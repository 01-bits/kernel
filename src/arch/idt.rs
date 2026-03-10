// use core::{arch::asm, mem::size_of, ptr::addr_of};

// #[derive(Debug)]
// #[repr(C)]
// pub struct InterruptStackFrame {
//     pub instruction_pointer: u64,
//     pub code_segment: u64,
//     pub cpu_flags: u64,
//     pub stack_pointer: u64,
//     pub stack_segment: u64,
// }

// #[derive(Clone, Copy)]
// #[repr(C, packed)]
// pub struct IdtEntry {
//     ptr_low: u16,  // Interrupt Handler Address
//     selector: u16, // Code Segment Selector
//     ist: u8,       // Interrupt Stack Table
//     access: u8,    // Access Flags / Type
//     ptr_mid: u16,
//     ptr_high: u32,
//     reserved: u32,
// }

// impl IdtEntry {
//     pub const fn empty() -> Self {
//         Self {
//             ptr_low: 0,
//             selector: 0,
//             ist: 0,
//             access: 0,
//             ptr_mid: 0,
//             ptr_high: 0,
//             reserved: 0,
//         }
//     }

//     pub fn set_handler(&mut self, handler: u64) {
//         self.ptr_low = handler as u16;
//         self.ptr_mid = (handler >> 16) as u16;
//         self.ptr_high = (handler >> 32) as u32;
//         self.selector = 0x08; // Pointing to our Kernel Code segment in GDT
//         self.access = 0x8E; // 0x8E = Present, Ring 0, Interrupt Gate
//     }
// }

// #[repr(C, packed)]
// pub struct IdtPtr {
//     limit: u16,
//     base: u64,
// }

// // #[repr(C, align(16))]
// // pub struct IdtTable([IdtEntry; 256]);

// static mut IDT: [IdtEntry; 256] = [IdtEntry::empty(); 256];

// pub fn init() {
//     unsafe {
//         // Load the IDT (currently empty, no handlers set)
//         // This allows the CPU to use the IDT descriptor table
//         // IDT[3].set_handler(breakpoint_handler as *const () as u64);
//         // IDT[8].set_handler(double_fault_handler as *const () as u64);
//         let handler = breakpoint_handler as u64;
//         IDT[3].set_handler(handler);

//         let ptr = IdtPtr {
//             limit: (size_of::<[IdtEntry; 256]>() - 1) as u16,
//             base: addr_of!(IDT) as u64,
//         };

//         asm!("lidt [{}]", in(reg) &ptr);
//         asm!("int 3");
//     }
// }

// // #[unsafe(no_mangle)]
// // pub extern "C" fn double_fault_handler() -> ! {
// //     crate::println!("\n!!! EXCEPTION: DOUBLE FAULT !!!");
// //     loop {
// //         unsafe {
// //             core::arch::asm!("hlt");
// //         }
// //     }
// // }

// pub extern "x86-interrupt" fn double_fault_handler(
//     stack_frame: InterruptStackFrame,
//     _error_code: u64,
// ) -> ! {
//     crate::println!("\nEXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
//     loop {
//         unsafe {
//             asm!("hlt");
//         }
//     }
// }

// // #[unsafe(no_mangle)]
// // pub extern "C" fn breakpoint_handler() {
// //     crate::println!("\nEXCEPTION: BREAKPOINT");
// // }

// #[unsafe(no_mangle)]
// pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
//     crate::println!("\nEXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
// }

// pub unsafe fn remap_pic() {
//     // Port constants
//     const PIC1_COMMAND: u8 = 0x20;
//     const PIC1_DATA: u8 = 0x21;
//     const PIC2_COMMAND: u8 = 0xA0;
//     const PIC2_DATA: u8 = 0xA1;

//     // Initialization Command
//     asm!("out 0x20, al", in("al") 0x11u8);
//     asm!("out 0xA0, al", in("al") 0x11u8);

//     // Set offsets (Map IRQ 0-7 to 32-39, and IRQ 8-15 to 40-47)
//     asm!("out 0x21, al", in("al") 0x20u8);
//     asm!("out 0xA1, al", in("al") 0x28u8);

//     // Wiring together
//     asm!("out 0x21, al", in("al") 0x04u8);
//     asm!("out 0xA1, al", in("al") 0x02u8);

//     // Set Environment mode
//     asm!("out 0x21, al", in("al") 0x01u8);
//     asm!("out 0xA1, al", in("al") 0x01u8);

//     // Mask all (disable for now, we'll enable keyboard specifically later)
//     asm!("out 0x21, al", in("al") 0xFDu8); // 0xFD enables IRQ 1 (keyboard)
//     asm!("out 0xA1, al", in("al") 0xFFu8);
// }

use core::{arch::asm, mem::size_of, ptr::addr_of};

use crate::drivers::ps2_kbd::{self};

#[derive(Debug)]
#[repr(C)]
pub struct InterruptStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IdtEntry {
    ptr_low: u16,
    selector: u16,
    ist: u8, // Interrupt Stack Table (Must be 0 if unused)
    access: u8,
    ptr_mid: u16,
    ptr_high: u32,
    reserved: u32, // Must be 0 in 64-bit
}

impl IdtEntry {
    pub const fn empty() -> Self {
        Self {
            ptr_low: 0,
            selector: 0,
            ist: 0,
            access: 0,
            ptr_mid: 0,
            ptr_high: 0,
            reserved: 0,
        }
    }

    pub fn set_handler(&mut self, handler: u64) {
        self.ptr_low = handler as u16;
        self.ptr_mid = (handler >> 16) as u16;
        self.ptr_high = (handler >> 32) as u32;
        self.selector = 0x08; // Matches GDT64_CODE in your boot.asm
        self.ist = 0;
        self.reserved = 0;
        self.access = 0x8E; // Present, Ring 0, Interrupt Gate
    }
}

// Ensure the IDT is 16-byte aligned for the CPU's internal fetch logic
#[repr(C, align(16))]
pub struct IdtTable([IdtEntry; 256]);

// Use a packed struct for the LIDT instruction to prevent Rust from adding padding
#[repr(C, packed)]
pub struct IdtPtr {
    limit: u16,
    base: u64,
}

static mut IDT: IdtTable = IdtTable([IdtEntry::empty(); 256]);

pub fn init() {
    unsafe {
        IDT.0[8].set_handler(double_fault_handler as u64);

        IDT.0[3].set_handler(breakpoint_handler as u64);
        IDT.0[0x21].set_handler(keyboard_handler as *const () as u64);

        let ptr = IdtPtr {
            limit: (size_of::<IdtTable>() - 1) as u16,
            base: addr_of!(IDT) as u64,
        };
        asm!("lidt [{}]", in(reg) &ptr);
    }
}

#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    crate::println!("\nEXCEPTION: DOUBLE FAULT");
    // crate::println!("\nEXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    // Print RIP on Row 15
    print_hex_raw(stack_frame.instruction_pointer, 5, 0);

    // Print CS on Row 16
    print_hex_raw(stack_frame.code_segment, 6, 0);

    // Print RSP on Row 17
    print_hex_raw(stack_frame.stack_pointer, 7, 0);
}

pub fn print_hex_raw(val: u64, row: usize, col: usize) {
    let buf = 0xb8000 as *mut u16;
    let chars = b"0123456789ABCDEF";
    let color = 0x0e00; // Yellow text

    for i in 0..16 {
        let nibble = (val >> ((15 - i) * 4)) & 0xf;
        let character = chars[nibble as usize] as u16;
        unsafe {
            // Calculate position: (row * 80 + col + i)
            let offset = (row * 80 + col + i) as isize;
            *buf.offset(offset) = color | character;
        }
    }
}

pub fn remap_pic() {
    unsafe {
        _remap_pic();
        asm!("sti");
    }
}

pub unsafe fn _remap_pic() {
    unsafe {
        // Initialization Command Word 1 (ICW1)
        asm!("out 0x20, al", in("al") 0x11u8); // ware up
        asm!("out 0xA0, al", in("al") 0x11u8); // ware up

        // ICW2: Set offsets (IRQ 0-7 -> 0x20, IRQ 8-15 -> 0x28)
        asm!("out 0x21, al", in("al") 0x20u8); // Master offset = 32 (0x20)
        asm!("out 0xA1, al", in("al") 0x28u8); // Slave offset = 40 (0x28)

        // ICW3: Wiring (Master/Slave connection)
        asm!("out 0x21, al", in("al") 0x04u8);
        asm!("out 0xA1, al", in("al") 0x02u8);

        // ICW4: 8086 mode
        asm!("out 0x21, al", in("al") 0x01u8);
        asm!("out 0xA1, al", in("al") 0x01u8);

        // Mask interrupts: Enable ONLY Keyboard (IRQ 1)
        // 0xFD = 11111101b (bit 1 is 0, so IRQ 1 is enabled)
        asm!("out 0x21, al", in("al") 0xFDu8);
        asm!("out 0xA1, al", in("al") 0xFFu8);
    }
}

#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn keyboard_handler(_stack_frame: InterruptStackFrame) {
    ps2_kbd::handle_interrupt();
}
