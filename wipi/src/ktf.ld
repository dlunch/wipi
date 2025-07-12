OUTPUT_FORMAT(binary)
ENTRY(__entrypoint)

MEMORY {
    ram (rwx) : ORIGIN = 0x100000, LENGTH = 64M
}

SECTIONS {
    .text : {
        *(.text.ktf_wipi_header);
        *(.text .text.*);
        . = ALIGN(4);
    } >ram = 0x00

    .rodata : {
        *(.rodata .rodata.*);
        . = ALIGN(4);
    } >ram = 0x00

    .bss : {
        *(.bss .bss.*);
        . = ALIGN(4);
    } >ram = 0x00

    .bss_info : {
        LONG(SIZEOF(.bss))
    } >ram = 0x00

    .shstrtab 0 : { *(.shstrtab) }

    /DISCARD/ : { *(*) }
}