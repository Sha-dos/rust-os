#![no_std]
#![no_main]

use core::panic::PanicInfo;
use x86_64::registers::control::Cr3;
use bootloader::{ entry_point, BootInfo };
use x86_64::structures::paging::{Page, PageTable};
use x86_64::VirtAddr;
extern crate alloc;
use alloc::boxed::Box;
use alloc::{rc, vec};
use alloc::rc::Rc;
use alloc::vec::Vec;
use rust_os::apps;
use rust_os::apps::hello;
use rust_os::kernel::Kernel;
use rust_os::task::{ Task, simple_executor::SimpleExecutor };
use rust_os::task::keyboard;
use rust_os::task::executor::Executor;

mod vga_buffer;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::allocator;
    use rust_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    rust_os::allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let mut executor = Executor::new();
    //executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::kernel_keypresses()));
    executor.spawn(Task::new(Kernel::init()));
    //executor.spawn(Task::new(Kernel::run()));
    executor.run();


    println!("It did not crash!");
    rust_os::hlt_loop();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

/*
#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    rust_os::init();

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    println!("It did not crash!");
    rust_os::hlt_loop();
}
*/
/// Called on a panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);

    rust_os::hlt_loop();
}