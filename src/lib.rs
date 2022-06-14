#![feature(abi_x86_interrupt)]
#![no_std]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(alloc_layout_extra)]
#![feature(wake_trait)]
#![feature(type_alias_impl_trait)]
#![feature(asm)]
#![feature(core_intrinsics)]

extern crate core;
extern crate alloc;
use core::panic::PanicInfo;

pub mod interrupts;
pub mod vga_buffer;
pub mod gdt;
pub mod serial;
pub mod memory;
pub mod allocator;
pub mod task;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout)
}