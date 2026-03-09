#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod spin;
pub use spin::*;
pub mod vga;
pub use vga::*;
pub mod idt;
pub use idt::*;
pub mod macros;
use core::{fmt::Write, panic::PanicInfo};

static HELLO: &[u8] = b"Hello Worldddd\n";

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    
    // If bss_start_addr is greater than (0x7e00 + (10 * 512)),
    // your current BIOS load is failing to reach the statics.
    // vg.write_fmt(format_args!("BSS Start: {:x}, Size: {}", bss_start_addr, bss_size)).unwrap();

    // unsafe {
    //     // Mark entry
    //     *(0xb8000 as *mut u16) = 0x0f31; // '1'
    //     core::arch::asm!("cli");
    // }

    // unsafe {
    //     *(0xb8002 as *mut u16) = 0x0f41;
    // } // 'A'
    // let mut vg = VgaWriter {
    //     row: 0,
    //     column: 0,
    //     buf: 0xb8000 as *mut u16,
    //     color: DEFAULT_COLOR,
    // };

    // unsafe {
    //     let raw_writer_ptr = core::ptr::addr_of!(WRITER) as *mut VgaWriter;
    //     (*raw_writer_ptr).write_fmt(format_args!("Test")).unwrap();
    // }
    // vg.write_byte(b'A', 0x0f);
    // vg.new_line();
    // let mut writer = WRITER.lock();
    // writer.color = 0x0f;
    // writer.write_fmt(format_args!("Hello"));
    // vg.write_fmt(format_args!("Hello"));
    // let mut writer = WRITER.lock();
    println!("Hello");
    // println!("Hello World");

    // let x = 5;
    // if x == 5 {
    //     unsafe {
    //         *(0xb8004 as *mut u16) = 0x0f42;
    //     } // 'B'
    // }

    // unsafe {
    //     *(0xb8006 as *mut u16) = 0x0f43;
    // } // 'C' - Before GDT init

    // unsafe {
    //     *(0xb8008 as *mut u16) = 0x0f32;
    // } // '2' - After GDT init

    // unsafe {
    //     *(0xb800a as *mut u16) = 0x0f49;
    // } // 'I' - Before IDT init

    // idt::init();

    // unsafe {
    //     *(0xb800c as *mut u16) = 0x0f33;
    // } // '3' - After IDT init

    // // Write SUCCESS message
    // unsafe {
    //     *(0xb800e as *mut u16) = 0x0f53; // S
    //     *(0xb8010 as *mut u16) = 0x0f4f; // O
    //     *(0xb8012 as *mut u16) = 0x0f4b; // K
    // }

    // unsafe {
    //     core::arch::asm!("int3")
    // }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
