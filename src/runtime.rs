
extern crate core;

use self::core::prelude::*;
use self::core::mem;
use self::core::mem::transmute;
use self::core::iter::Iterator;
use self::core::option::{Some, Option, None};
use libc::console::println;
use libc;

#[lang = "begin_unwind"]
fn begin_unwind(args: &self::core::fmt::Arguments,
                      file: &str,
                      line: uint) -> ! {
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

    // libc::origin::cprintf("topheap: %x\n\x00".as_ptr(), topheap);
    let remain = topheap % _align;
    let bg = if remain == 0 { topheap } else { topheap - remain + size} ;

    // increase top
    topheap = bg + size;

    (bg) as *mut u8
}

#[lang="exchange_free"]
unsafe fn deallocate(ptr: *mut u8, _size: uint, _align: uint) {
    // FIXME: do nothing!
    // libc::origin::cprintf("dealloc at: 0x%x\n\x00".as_ptr(), ptr);
}
