use crate::{arch::idt::InterruptStackFrame, print_info, shell};

pub fn handle_interrupt() {
    unsafe {
        let scancode: u8;
        core::arch::asm!("in al, 0x60", out("al") scancode);

        // Bit 7 (0x80) is set if the key was released.
        // We only want to print on the "Press" event.
        if scancode & 0x80 == 0 {
            let key = match scancode {
                0x10 => Some('a'),
                0x11 => Some('z'),
                0x12 => Some('e'),
                0x13 => Some('r'),
                0x14 => Some('t'),
                0x15 => Some('y'),
                0x16 => Some('u'),
                0x17 => Some('i'),
                0x18 => Some('o'),
                0x19 => Some('p'),
                0x1E => Some('q'),
                0x1F => Some('s'),
                0x20 => Some('d'),
                0x21 => Some('f'),
                0x22 => Some('g'),
                0x23 => Some('h'),
                0x24 => Some('j'),
                0x25 => Some('k'),
                0x26 => Some('l'),
                0x27 => Some('m'),
                0x2C => Some('w'),
                0x2D => Some('x'),
                0x2E => Some('c'),
                0x2F => Some('v'),
                0x30 => Some('b'),
                0x31 => Some('n'),
                0x39 => Some(' '),  // Space
                0x1C => Some('\n'), // Enter
                _ => None,
            };

            if let Some(c) = key {
                print_info!("{}", c);
                // shell::p
            }
        }

        // Send End of Interrupt (EOI) to the Master PIC
        core::arch::asm!("out 0x20, al", in("al") 0x20u8);
    }
}
