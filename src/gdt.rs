use core::{arch::asm, ptr::addr_of};

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
            limit_low: 0xFFFF,
            base_low: 0,
            base_mid: 0,
            access,
            limit_high_flags: 0x0F | (flags & 0xF0), // Only the high 4 bits are flags
            base_high: 0,
        }
    }
}

#[repr(C, align(16))]
pub struct GdtTable([GdtEntry; 3]);

pub static mut GDT: GdtTable = GdtTable([
    GdtEntry::new(0, 0),       // Null descriptor (Required by x86 architecture)
    GdtEntry::new(0x9A, 0x20), // Kernel Code segment (Executable, Readable, Accessed)
    GdtEntry::new(0x92, 0x00), // Kernel Data segment (Readable, Accessed)
]);

#[repr(C, packed(2))] // Forces the 2-byte limit and 8-byte base to sit tight
pub struct GdtDescriptor {
    pub limit: u16,
    pub base: u64,
}

pub fn init() {
    unsafe {
        // Create the 10-byte GDT descriptor manually on the stack
        let mut gdt_ptr = [0u8; 10];
        let limit = (core::mem::size_of::<GdtTable>() - 1) as u16;
        let base = addr_of!(GDT) as u64;

        gdt_ptr[0..2].copy_from_slice(&limit.to_le_bytes());
        gdt_ptr[2..10].copy_from_slice(&base.to_le_bytes());

        asm!(
            "lgdt [{0}]",            // Load from our manual byte array
            "push 0x08",             // Code selector
            "lea rax, [2f]",
            "push rax",
            "retfq",                 // If the GDT at 'base' is valid, this WILL work
            "2:",
            "mov ax, 0x10",
            "mov ds, ax",
            "mov es, ax",
            "mov ss, ax",
            in(reg) &gdt_ptr,
            out("rax") _,
            options(nostack)
        );
    }
}