#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod drivers;
pub mod shell;
pub mod sync;
pub use drivers::vga::*;
pub mod arch;
pub mod macros;
use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello Worldddd\n";

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    println!("start booting");
    arch::gdt::init();
    arch::idt::init();
    println!("IDT an GDT done");
    arch::idt::remap_pic();
    println!("Keyboard ready. Type something!");

    
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
