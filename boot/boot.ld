OUTPUT_ARCH(i386)
SECTIONS
{
  . = 0x7c00;           /* begin location from BIOS */
  .bstext : { *(.bstext) }
  .bsdata : { *(.bsdata) }
  _GLOBAL_OFFSET_TABLE_ = .;
  . = 0x7e00 - 0x2;
  .bssign : { *(.bssignature) }
  .text : { *(.text) }
}

