.section .text.ktf_wipi_header, "ax"
.global __start
__start:
    b after_data

.byte 0xC0 // Some kind of magic
.byte 0x46
.byte 0x24
.byte 0x02
.byte 0x04
.byte 0x20
.byte 0x01
.byte 0x00
.byte 0x02
.byte 0x00

after_data:
    push {{LR}}
    adr R0, wipi_exe_header
    pop {{R3}}
    bx R3

.section .text, "ax"

str_wipi_exe:
.string "WIPI_exe"

str_exe_interface:
.string "ExeInterface"

.align 4
wipi_exe_interface_methods:
.word 0 // unk
.word 0 // unk
.word 0 // init
.word 0 // get_default_dll
.word 0 // get_class
.word 0 // unk
.word 0 // unk

.align 4
wipi_exe_interface_header:
.word wipi_exe_interface_methods
.word str_exe_interface
.word 1 // unk
.word 0 // unk
.word 0 // unk
.word 0 // unk
.word 0 // unk
.word 0 // unk

.align 4
wipi_exe_header:
.word wipi_exe_interface_header
.word str_wipi_exe
.word 1 // unk
.word 0 // unk
.word 0 // unk
.word main // init
.word 0 // unk
.word 0 // unk
.word 0 // fini
.word 0 // unk
