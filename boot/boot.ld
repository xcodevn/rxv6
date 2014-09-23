ENTRY(start)
OUTPUT_ARCH(i386)
OUTPUT_FORMAT(binary)

SECTIONS
{
  . = 0x7c00;           /* begin location from BIOS */
  .text : { *(.text) }
  .data : { *(.data) }
  . = 0x7e00 - 0x2;
  .bssign : { *(.bssignature) }
  /DISCARD/ : { * (*) }
}

