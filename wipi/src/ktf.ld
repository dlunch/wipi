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
        __bss_start = ABSOLUTE(.);

        *(.bss .bss.*);
        . = ALIGN(4);

        __bss_end = ABSOLUTE(.);
    } >ram = 0x00

    __bss_size = __bss_end - __bss_start;

    .bss_info : {
        LONG(__bss_size)
    } >ram = 0x00

    .shstrtab 0 : { *(.shstrtab) }

    /DISCARD/ : { *(*) }
}