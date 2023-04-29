use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Deref;
use crate::apps::App;
use crate::apps::hello::HelloApp;
use crate::{print, println};
use crate::disk::{BLOCK_SIZE, Disk, FILE_PATH};
use crate::task::keyboard::kernel_keypresses;

pub struct Kernel;

impl Kernel {
    pub async fn init() {
        Self::print_console();
    }

    pub async fn run() {
        kernel_keypresses();
    }

    pub fn print_console() {
        print!(">");
    }

    pub async fn run_command(command: &str) {
        let args: Vec<&str> = command.split(' ').collect();

        match args[0] {
            "exec" => {
                if Kernel::is_exec(args[1]) {
                    crate::apps::run_app(args[1]).await
                } else {
                    println!("Error: Unknown application: {}", args[1]);
                }
            }

            "ls" => {

            }

            "cat" => {
                let mut disk = Disk::open(FILE_PATH);
                let mut buffer = [0u8; BLOCK_SIZE];
                match disk.read(0, &mut buffer) {
                    Ok(bytes_read) => {
                        println!("{}", bytes_read);
                    }
                    Err(err) => {
                        // Handle error
                    }
                }
                let buffer = b"Hello, world!\n";
                match disk.write(1, buffer) {
                    Ok(bytes_written) => {
                        if bytes_written != buffer.len() {
                            // Handle incomplete write
                        }
                    }
                    Err(err) => {
                        // Handle error
                    }
                }
            }

            &_ => { println!("Error: Unknown command: {}", command);}
        }
    }

    pub fn is_exec(maybe_exec: &str) -> bool {
        let mut apps: Vec<Box<dyn App>> = Vec::new();
        apps.push(Box::new(HelloApp));
        for app in apps.iter() {
            if app.name() == maybe_exec.to_string() {
                 return true;
            }
        }

        false
    }
}

