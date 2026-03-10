use core::arch::asm;

/// This module defines the Global Descriptor Table (GDT) for the kernel.
/// The GDT is a data structure used by x86 processors to define the characteristics of the various memory segments used in protected mode.
/// Each entry in the GDT describes a segment, including its base address, limit, access
/// rights, and other attributes. The GDT is essential for setting up the memory management and protection mechanisms of the operating system.
#[repr(C, packed)]
pub struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    limit_high_flags: u8,
    base_high: u8,
}

impl GdtEntry {
    /// Creates a new GDT entry with the specified access and flags.
    /// The `access` byte defines the access rights for the segment, while the `flags` byte contains additional attributes such as the granularity and size of the segment.
    /// The base address and limit are set to zero for simplicity, as they are not used in this basic GDT setup.
    pub const fn new(access: u8, flags: u8) -> Self {
        GdtEntry {
            limit_low: 0,
            base_low: 0,
            base_mid: 0,
            access,
            limit_high_flags: flags & 0xF0, // Only the high 4 bits are flags
            base_high: 0,
        }
    }
}

/// The GDT pointer structure, which is used to load the GDT into the CPU.
/// It contains the size of the GDT (limit) and the base address where the GDT is located in memory.
/// The `repr(C, packed)` attribute ensures that the structure is laid out in memory without any padding,
/// which is crucial for the CPU to correctly interpret the GDT pointer
#[repr(C, packed)]
pub struct GdtPtr {
    limit: u16,
    base: u64,
}

static GDT: [GdtEntry; 3] = [
    GdtEntry::new(0, 0),       // Null descriptor (Required by x86 architecture)
    GdtEntry::new(0x9A, 0x20), // Kernel Code segment (Executable, Readable, Accessed)
    GdtEntry::new(0x92, 0x00), // Kernel Data segment (Readable, Accessed)
];

pub fn init() {
    let ptr = GdtPtr {
        limit: size_of::<[GdtEntry; 3]>() as u16 - 1,
        base: &GDT as *const _ as u64,
    };

    unsafe {
        load_gdt(&ptr);
    }
}

pub unsafe fn load_gdt(ptr: &GdtPtr) {
    unsafe {
        asm!(
            "lgdt [{0}]",
            "push 0x08",                 // Push the new Code Segment selector (8 bytes in)
            "lea {tmp}, [2f]",           // Load the address of the label '2'
            "push {tmp}",                // Push that address
            "retfq",                     // This pops RIP then CS. CPU jumps to '1:'
            "2:",
            "mov ax, 0x10",              // 0x10 is the Data Segment (16 bytes in)
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "mov ss, ax",
            in(reg) ptr,
            tmp = out(reg) _,
        );
    }
}
