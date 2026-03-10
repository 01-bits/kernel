ENTRY(start)
OUTPUT_FORMAT(elf64-x86-64)

SECTIONS
{
    . = 0x7c00;

    .boot :
    {
        *(.boot_entry)
    }

    /* Fill to the end of the first sector */
    . = 0x7c00 + 510;
    .sig :
    {
        SHORT(0xAA55);
    }
    

    /* Rust code starts here (Sector 2 on the disk) */
    . = 0x7e00;
    /* .kernel : AT(0x7e00) */
    /* {
        *(.text .text.*)
        *(.rodata .rodata.*)
        *(.data .data.*)
        *(.bss .bss.*)
        *(COMMON)
    } */
    .text : { *(.text .text.*) }
    .rodata : { *(.rodata .rodata.*) }
    .data : { *(.data .data.*) }
    
    .bss : 
    { 
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    }

    /DISCARD/ : { *(.comment) *(.eh_frame) *(.note*) }
}