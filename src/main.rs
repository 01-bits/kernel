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
    unsafe { core::arch::asm!("cli"); }
    // Write a White '1' to indicate: Entry reached
    unsafe { *(0xb8000 as *mut u16) = 0x0f31; }

    let x = 5;
    if x == 5 {
         unsafe { *(0xb8002 as *mut u16) = 0x0f41; } // See 'A'
    }

    // gdt::init();
    // Write a White '2' to indicate: GDT done
    // unsafe { *(0xb8002 as *mut u16) = 0x0f32; }

    idt::init();
    // // Write a White '3' to indicate: IDT done
    unsafe { *(0xb8004 as *mut u16) = 0x0f33; }

    // TRIGGER THE INTERRUPT
    // unsafe { core::arch::asm!("int3"); }

    // // Write a White '4' to indicate: Returned from Interrupt
    // unsafe { *(0xb8006 as *mut u16) = 0x0f34; }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
