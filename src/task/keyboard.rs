use alloc::string::{String, ToString};
use alloc::{format, vec};
use alloc::vec::Vec;
use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use crate::println;
use core::{ pin::Pin, task::{ Poll, Context } };
use core::intrinsics::powf32;
use bootloader::bootinfo::MemoryRegionType::Kernel;
use futures_util::task::AtomicWaker;
use futures_util::stream::Stream;
use futures_util::stream::StreamExt;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1, KeyCode, KeyState};
use crate::apps::App;
use crate::print;
use crate::task::executor::Executor;
use crate::task::Task;


static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new should only be called once");
        ScancodeStream { _private: () }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE.try_get().expect("not initialized");

        if let Ok(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Ok(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            Err(crossbeam_queue::PopError) => Poll::Pending,
        }
    }
}
//endregion

//region functions
pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            println!("WARNING: scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        println!("WARNING: scancode queue uninitialized");
    }
}

pub async fn init() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);
}

pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1,
    HandleControl::Ignore);

    /*
    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(Key) = keyboard.process_keyevent(key_event) {
                match Key {
                    DecodedKey::Unicode(character) => print!("CHAR: {}", character),
                    //DecodedKey::RawKey(key) => print!("{:?}", key),
                    DecodedKey::RawKey(key) => {
                        println!("Hello App!");
                        if key == KeyCode::Enter {
                            println!("Enter Pressed");
                            crate::apps::hello::run();
                        }
                    },
                }
            }
        }
    }*/

    while let Some(scancode) = scancodes.next().await {
        /*
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                if key == DecodedKey::RawKey(KeyCode::ShiftLeft) {
                    println!("Hello Keypress");
                }
            }
        }*/

        if let key_code = keyboard.add_byte(scancode).unwrap().unwrap() {
            if key_code.code == KeyCode::Enter && key_code.state == KeyState::Down {
                println!("Running App");

            }
        }
    }
}

pub async fn kernel_keypresses() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1,
                                     HandleControl::Ignore);

    let mut chars: Vec<char> = Vec::new();

    /*
    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(Key) = keyboard.process_keyevent(key_event) {
                match Key {
                    DecodedKey::Unicode(character) => {
                        print!("{}", character);
                        chars.push(character)
                    },
                    DecodedKey::RawKey(key) => {
                        if key == KeyCode::Enter {
                            let mut string = String::new();

                            for char in &chars {
                                string.push(*char)
                            }

                            crate::kernel::Kernel::run_command(string.as_str());
                        }
                    },
                }
            }
        }
    }*/

    while let Some(scancode) = scancodes.next().await {
        if let key_code = keyboard.add_byte(scancode).unwrap().unwrap() {
            if let Some(Key) = keyboard.process_keyevent(key_code.clone()) {
                match Key {
                    DecodedKey::Unicode(character) => {
                        print!("{}", character);
                        chars.push(format!("{}", character).chars().nth(0).unwrap());
                    }
                    //DecodedKey::RawKey(key) => print!("{:?}", key),
                    DecodedKey::RawKey(key) => {},
                }
            }

            if key_code.code == KeyCode::Backspace && key_code.state == KeyState::Down {
                chars.pop();
                chars.pop();
            }

            if key_code.code == KeyCode::Enter && key_code.state == KeyState::Down {
                let mut string = String::new();

                for char in &chars {
                    string.push(*char)
                }
                println!("Running: {}", string[0..string.len() - 1].to_string());

                crate::kernel::Kernel::run_command(string[0..string.len() - 1].to_string().as_str()).await;

                chars.clear();

                crate::kernel::Kernel::print_console();
            }
        }
    }
}