OUTPUT_ARCH(i386)
OUTPUT_FORMAT(binary)
SECTIONS
{
  . = 0x7c00;           /* begin location from BIOS */
  .bstext : { *(.bstext) }
  .bsdata : { *(.bsdata) }
  . = 0x7e00 - 0x2;
  .bssign : { *(.bssignature) }
}

