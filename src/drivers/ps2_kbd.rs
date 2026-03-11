use crate::{arch::idt::InterruptStackFrame, print_info, println, shell};

pub fn handle_interrupt() {
    unsafe {
        let scancode: u8;
        core::arch::asm!("in al, 0x60", out("al") scancode);

        // Bit 7 (0x80) is set if the key was released.
        // We only want to print on the "Press" event.
        if scancode & 0x80 == 0 {
            let key = match scancode {
                // Letters (your existing ones + missing)
                0x10 => Some('q'),
                0x11 => Some('s'),
                0x12 => Some('e'),
                0x13 => Some('d'),
                0x14 => Some('r'),
                0x1E => Some('a'),
                0x1F => Some('w'),
                0x20 => Some('z'),
                0x21 => Some('x'),
                0x22 => Some('c'),
                0x23 => Some('v'),
                0x24 => Some('b'),
                0x25 => Some('n'),
                0x26 => Some('m'),
                0x15 => Some('t'),
                0x16 => Some('y'),
                0x17 => Some('u'),
                0x18 => Some('i'),
                0x19 => Some('o'),
                0x27 => Some(','),
                0x28 => Some('.'),
                0x29 => Some('/'),
                0x2C => Some(' '),

                // Numbers & symbols
                0x02 => Some('1'),
                0x03 => Some('2'),
                0x04 => Some('3'),
                0x05 => Some('4'),
                0x06 => Some('5'),
                0x07 => Some('6'),
                0x08 => Some('7'),
                0x09 => Some('8'),
                0x0A => Some('9'),
                0x0B => Some('0'),
                0x0C => Some('-'),
                0x0D => Some('='),
                0x0E => Some(8u8 as char), // Backspace!
                0x0F => Some('`'),
                0x27 => Some(';'),
                0x2B => Some('\''),
                0x2D => Some('['),
                0x2E => Some('\\'),
                0x2F => Some(']'),

                // Control
                0x1C => Some('\n'), // Enter
                _ => None,
            };

            if let Some(c) = key {
                // print_info!("{}", c);
                shell::ordis::SHELL.lock().handle_char(c);
                // shell::p
            }
        }

        // Send End of Interrupt (EOI) to the Master PIC
        core::arch::asm!("out 0x20, al", in("al") 0x20u8);
    }
}
