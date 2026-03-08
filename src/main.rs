#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod spin;
pub use spin::*;
pub mod vga;
pub use vga::*;
pub mod gdt;
pub use gdt::*;
pub mod idt;
pub use idt::*;
pub mod macros;
use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello Worldddd\n";

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    unsafe {
        // Mark entry
        *(0xb8000 as *mut u16) = 0x0f31; // '1'
        core::arch::asm!("cli");
    }

    unsafe {
        *(0xb8002 as *mut u16) = 0x0f41;
    } // 'A'

    let x = 5;
    if x == 5 {
        unsafe {
            *(0xb8004 as *mut u16) = 0x0f42;
        } // 'B'
    }

    unsafe {
        *(0xb8006 as *mut u16) = 0x0f43;
    } // 'C' - Before GDT init

    gdt::init();

    unsafe {
        *(0xb8008 as *mut u16) = 0x0f32;
    } // '2' - After GDT init

    unsafe {
        *(0xb800a as *mut u16) = 0x0f49;
    } // 'I' - Before IDT init

    idt::init();

    unsafe {
        *(0xb800c as *mut u16) = 0x0f33;
    } // '3' - After IDT init

    // Write SUCCESS message
    unsafe {
        *(0xb800e as *mut u16) = 0x0f53; // S
        *(0xb8010 as *mut u16) = 0x0f4f; // O
        *(0xb8012 as *mut u16) = 0x0f4b; // K
    }

    // unsafe {
    //     core::arch::asm!("int3")
    // }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
