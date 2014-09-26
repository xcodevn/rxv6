
#include <inc/stdio.h>
#include <inc/string.h>
#include "backtrace/backtrace.h"
#include "backtrace/internal.h"

extern void main();
extern void* malloc(int s);

void error_callback();

extern char debuginfo_begin[];
extern char debuginfo_end[];
extern char debugline_begin[];
extern char debugline_end[];
extern char debugabbrev_begin[];
extern char debugabbrev_end[];
extern char debugranges_begin[];
extern char debugranges_end[];
extern char debugstr_begin[];
extern char debugstr_end[];

char data[1024];
static int base_address= 0xf0100000;

void error_callback(void * a, const char * b, int c) {
}




void debug_init() {
 struct backtrace_state * state = backtrace_create_state(
                                    "src/kernel.elf", 0,
                                    error_callback, data);

  fileline fn;

  backtrace_dwarf_add (state, base_address,
          (const unsigned char*) debuginfo_begin,
          debuginfo_end - debuginfo_begin,
          (const unsigned char*) debugline_begin,
          debugline_end - debugline_begin,
          (const unsigned char*) debugabbrev_begin,
          debugabbrev_end - debugabbrev_begin,
          (const unsigned char*) debugranges_begin,
          debugranges_end - debugranges_begin,
          (const unsigned char*) debugstr_begin,
          debugstr_end - debugstr_begin,
          0,
			    error_callback, 
          data, 
          &fn);
}

void i386_init() {

	extern char edata[], end[];

	// Before doing anything else, complete the ELF loading process.
	// Clear the uninitialized global data (BSS) section of our program.
	// This ensures that all static/global variables start out zero.
	// memset(edata, 0, end - edata);

  debug_init();

  // call `main` function written in Rust
   main();
}

void _Unwind_Resume () {
  while(true) /* do nothing */ ;
}
