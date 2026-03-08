use core::{arch::asm, ptr::addr_of};

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
    ptr_low: u16,  // Interrupt Handler Address
    selector: u16, // Code Segment Selector
    ist: u8,       // Interrupt Stack Table
    access: u8,    // Access Flags / Type
    ptr_mid: u16,
    ptr_high: u32,
    reserved: u32,
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
        self.selector = 0x08; // Pointing to our Kernel Code segment in GDT
        self.access = 0x8E; // 0x8E = Present, Ring 0, Interrupt Gate
    }
}

#[repr(C, packed)]
pub struct IdtPtr {
    limit: u16,
    base: u64,
}

// #[repr(C, align(16))]
// pub struct IdtTable([IdtEntry; 256]);

static mut IDT: [IdtEntry; 256] = [IdtEntry::empty(); 256];

pub fn init() {
    // unsafe {
    //     IDT[3].set_handler(breakpoint_handler as *const () as u64);
    //     IDT[8].set_handler(double_fault_handler as *const () as u64);
    //     let ptr = IdtPtr {
    //         limit: size_of::<[IdtEntry; 256]>() as u16 - 1,
    //         base: addr_of!(IDT) as u64,
    //     };

    //     asm!("lidt [{}]", in(reg) &ptr);
    // }
    unsafe {
        let mut local_idt = [IdtEntry::empty(); 16];

        local_idt[3].set_handler(breakpoint_handler as u64);

        let ptr = IdtPtr {
            limit: (16 * 16 - 1) as u16,
            base: (&local_idt as *const _) as u64,
        };

        asm!("lidt [{}]", in(reg) &ptr);

        // Write '3' to screen directly from here to see if we survived
        *(0xb8004 as *mut u16) = 0x0f33;
    }
}

// #[unsafe(no_mangle)]
// pub extern "C" fn double_fault_handler() -> ! {
//     crate::println!("\n!!! EXCEPTION: DOUBLE FAULT !!!");
//     loop {
//         unsafe {
//             core::arch::asm!("hlt");
//         }
//     }
// }

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    crate::println!("\nEXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

// #[unsafe(no_mangle)]
// pub extern "C" fn breakpoint_handler() {
//     crate::println!("\nEXCEPTION: BREAKPOINT");
// }

#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    crate::println!("\nEXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

pub unsafe fn remap_pic() {
    // Port constants
    const PIC1_COMMAND: u8 = 0x20;
    const PIC1_DATA: u8 = 0x21;
    const PIC2_COMMAND: u8 = 0xA0;
    const PIC2_DATA: u8 = 0xA1;

    // Initialization Command
    asm!("out 0x20, al", in("al") 0x11u8);
    asm!("out 0xA0, al", in("al") 0x11u8);

    // Set offsets (Map IRQ 0-7 to 32-39, and IRQ 8-15 to 40-47)
    asm!("out 0x21, al", in("al") 0x20u8);
    asm!("out 0xA1, al", in("al") 0x28u8);

    // Wiring together
    asm!("out 0x21, al", in("al") 0x04u8);
    asm!("out 0xA1, al", in("al") 0x02u8);

    // Set Environment mode
    asm!("out 0x21, al", in("al") 0x01u8);
    asm!("out 0xA1, al", in("al") 0x01u8);

    // Mask all (disable for now, we'll enable keyboard specifically later)
    asm!("out 0x21, al", in("al") 0xFDu8); // 0xFD enables IRQ 1 (keyboard)
    asm!("out 0xA1, al", in("al") 0xFFu8);
}
