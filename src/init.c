
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

static struct backtrace_state * state;


struct debug_info {
    const char* file_name;
    int   file_name_len;
    const char* func_name;
    int   func_name_len;
    int   file_line;
};

static fileline fn;
extern void cons_init();

int callback_fn (void *data, uintptr_t pc,
					const char *filename, int lineno,
					const char *function) {

    struct debug_info * dt = (struct debug_info*)data;
    if (filename != NULL) dt->file_name = filename;
      else dt->file_name = "<unk>";

    dt->file_name_len = strlen(dt->file_name);
    if (function) dt->func_name = function;
      else dt->func_name = "<unk>";
    dt->func_name_len = strlen( dt->func_name );
    dt->file_line = lineno;

    return 0;
}

void fileline_debug(unsigned int pc, struct debug_info* data) {
  fn(state, pc,  callback_fn, error_callback, data);
}

void debug_init() {
  state = backtrace_create_state( "src/kernel.elf", 0, error_callback, data);

  backtrace_initialize(state, 0, error_callback,  data, &fn);

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
