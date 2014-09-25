
#include <inc/string.h>

extern void main();

void i386_init() {

	extern char edata[], end[];

	// Before doing anything else, complete the ELF loading process.
	// Clear the uninitialized global data (BSS) section of our program.
	// This ensures that all static/global variables start out zero.
	memset(edata, 0, end - edata);

    // call `main` function written in Rust
    main();
}

void _Unwind_Resume () {
  while(true) /* do nothing */ ;
}
