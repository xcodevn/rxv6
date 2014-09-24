
extern crate core;

use self::core::prelude::*;
use self::core::mem;
use self::core::mem::transmute;
use libc::console::println;
use libc;

#[lang = "begin_unwind"]
fn begin_unwind(args: &self::core::fmt::Arguments,
                        file: *const u8, len:uint, line: uint) -> ! {

        // error message also be catched by vim!
        unsafe { libc::origin::cprintf("%s:%d:0  %d:0 error  begin_unwind \x00".as_ptr(), file, line , line) ; } 
        loop {}
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}


// Linearly increase the address
//
#[lang="exchange_malloc"]
unsafe fn allocate(size: uint, _align: uint) -> *mut u8 {

    static mut topheap:uint = 0;

    if topheap == 0 { topheap = libc::origin::heapbase(); }

    libc::origin::cprintf("\ntopheap: %x\n\x00".as_ptr(), topheap);
    let remain = topheap % _align;
    let bg = if remain == 0 { topheap } else { topheap - remain + size} ;

    // increase top
    topheap = bg + size;

    (bg) as *mut u8
}

#[lang="exchange_free"]
unsafe fn deallocate(ptr: *mut u8, _size: uint, _align: uint) {
    // FIXME: do nothing!
     libc::origin::cprintf("\ndealloc at: 0x%x\n\x00".as_ptr(), ptr);
}
