// defines ktf wipi binary entrypoint

.section .text.ktf_wipi_header, "ax"
.global __entrypoint
__entrypoint:
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
    b start
