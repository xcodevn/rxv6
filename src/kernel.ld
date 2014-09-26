/* Simple linker script for the JOS kernel.
   See the GNU ld 'info' manual ("info ld") to learn the syntax. */

OUTPUT_FORMAT("elf32-i386", "elf32-i386", "elf32-i386")
OUTPUT_ARCH(i386)
ENTRY(_start)

SECTIONS
{
	/* Link the kernel at this address: "." means the current address */
	. = 0xF0100000;

	/* AT(...) gives the load address of this section, which tells
	   the boot loader where to load the kernel in physical memory */
	.text : AT(0x100000) {
		*(.text .stub .text.* .gnu.linkonce.t.*)
	}

	PROVIDE(etext = .);	/* Define the 'etext' symbol to this value */

	.rodata : {
		*(.rodata .rodata.* .gnu.linkonce.r.* .data.rel.*);

        PROVIDE(debuginfo_begin = .);
        *(.debug_info);
        PROVIDE(debuginfo_end = .);

        PROVIDE(debugline_begin = .);
        *(.debug_line);
        PROVIDE(debugline_end = .);

        PROVIDE(debugabbrev_begin = .);
        *(.debug_abbrev);
        PROVIDE(debugabbrev_end = .);

        PROVIDE(debugranges_begin = .);
        *(.debug_ranges);
        PROVIDE(debugranges_end = .);

        PROVIDE(debugstr_begin = .);
        *(.debug_str);
        PROVIDE(debugstr_end = .);
	}

	/* Include debugging information in kernel memory */
	.stab : {
		PROVIDE(__STAB_BEGIN__ = .);
		*(.stab);
		PROVIDE(__STAB_END__ = .);
		BYTE(0)		/* Force the linker to allocate space
				   for this section */
	}

	.stabstr : {
		PROVIDE(__STABSTR_BEGIN__ = .);
		*(.stabstr);
		PROVIDE(__STABSTR_END__ = .);
		BYTE(0)		/* Force the linker to allocate space
				   for this section */
	}

	/* Adjust the address for the data segment to the next page */
	. = ALIGN(0x1000);

	/* The data segment */
	.data : {
		*(.data)
	}

	/* The data segment */
	.heapdata : {
		*(.heapdata)
	}
	.bss : {
      PROVIDE(edata = .);
		*(.bss);
      PROVIDE(end = .);
	}


	/DISCARD/ : {
		*(.eh_frame .note.GNU-stack)
	}
    
}
