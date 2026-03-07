use core::arch::asm;

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

static mut IDT: [IdtEntry; 256] = [IdtEntry::empty(); 256];

pub fn init() {
    unsafe {
        IDT[8].set_handler(double_fault_handler as u64);
        let ptr = IdtPtr {
            limit: size_of::<[IdtEntry; 256]>() as u16 - 1,
            base: &IDT as *const _ as u64,
        };

        asm!("lidt [{}]",in(reg) &ptr);
    }
}

pub extern "C" fn double_fault_handler() -> ! {
    crate::println!("\n!!! EXCEPTION: DOUBLE FAULT !!!");
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}
