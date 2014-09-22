# Author: TTN
# Data: 21-09-2014

.extern main

.code16

.globl _start
.globl signature

.section ".bstext", "a"

.align
_start:
  xorw  %ax, %ax
  movw  %ax, %ds
  movw  %ax, %es
  movw  %ax, %ss
  movw $0x7bfe, %sp
  movb  $0x2, %ah
  movb $24 , %al
  movb $0  , %ch
  movb $2  , %cl
  movb $0  , %dh
  movw $0x7e00, %bx
  int $0x13
  jc idle

  cli
  cld

seta20.1:
  inb     $0x64,%al               # Wait for not busy
  testb   $0x2,%al
  jnz     seta20.1

  movb    $0xd1,%al               # 0xd1 -> port 0x64
  outb    %al,$0x64

seta20.2:
  inb     $0x64,%al               # Wait for not busy
  testb   $0x2,%al
  jnz     seta20.2

  movb    $0xdf,%al               # 0xdf -> port 0x60
  outb    %al,$0x60

  lgdt gdtr
  movl %cr0, %eax
  orl  $01 , %eax
  movl %eax, %cr0
  ljmp $0x08, $_protected_mode

idle:
  jmp  idle

_protected_mode:
.code32

  movl $0x10, %eax
  mov %eax, %ds
  mov %eax, %es
  mov %eax, %fs
  mov %eax, %gs
  mov %eax, %ss

  movl $0x0, %gs:0x30

  movl $0x7bfc, %esp
  call main
  nop
loop:
  jmp loop

.text
.globl __morestack

__morestack:
  jmp __morestack

_GLOBAL_OFFSET_TABLE_:
  .byte 0x0

.section ".bsdata", "a" 

.align

gdtr:
    .word (gdt_end - gdt) + 1  
    .long gdt                 

idtr:
    .word 0
    .long 0

gdt:
    .long 0,0
    .word 0xffff       # limit 0:15
    .word 0x0000       # base 0:15
    .byte 0x00         # base 16:23
    .byte 0b10011010   # access byte - code
    .byte 0x4f         # flags/(limit 16:19). flag is set to 32 bit protected mode
    .byte 0x00         # base 24:31
    .word 0xffff       # limit 0:15
    .word 0x0000       # base 0:15
    .byte 0x00         # base 16:23
    .byte 0b10010010   # access byte - data
    .byte 0x4f         # flags/(limit 16:19). flag is set to 32 bit protected mode
    .byte 0x00         # base 24:31
gdt_end:

author:
  .ascii "xcodevn"

.section ".bssignature", "a"
signature:
  .word 0xaa55


