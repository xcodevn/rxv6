# Author: TTN
# Data: 21-09-2014

.code16

.globl _start
.globl signature
.section ".bstext", "a"

.align
_start:
  mov $0xb800, %ax
  mov %ax, %ds
  movb $'A', 0
  movb $0x1e, 1

  mov $04, %ax
  nop

idle:
  jmp  idle

.section ".bsdata", "a" 
.align

author:
  .ascii "xcodevn"

.section ".bssignature", "a"
signature:
  .word 0xaa55
