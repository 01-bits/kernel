#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod spin;
pub use spin::*;
pub mod vga;
pub use vga::*;
pub mod idt;
pub use idt::*;
pub mod gdt;
pub mod macros;
use core::arch::asm;
use core::{fmt::Write, panic::PanicInfo};

static HELLO: &[u8] = b"Hello Worldddd\n";

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    println!("start booting");
    gdt::init();
    idt::init();
    println!("IDT an GDT done");
    idt::remap_pic();
    println!("Keyboard ready. Type something!");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
