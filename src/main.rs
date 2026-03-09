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
    
    unsafe {
        // Mark entry
        *(0xb8000 as *mut u16) = 0x0f31; // '1'
        core::arch::asm!("cli");
    }

    
    println!("start booting");
    idt::init();
    println!("idt done");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
