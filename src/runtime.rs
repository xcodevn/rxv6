
extern crate core;

use self::core::prelude::*;
use self::core::mem;
use self::core::mem::transmute;
use self::core::iter::Iterator;
use self::core::option::{Some, Option, None};

#[lang = "begin_unwind"]
extern fn begin_unwind(args: &self::core::fmt::Arguments,
                      file: &str,
                      line: uint) -> ! {
        loop {}
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang="exchange_malloc"]
unsafe fn allocate(size: uint, _align: uint) -> *mut u8 {
    (0) as *mut u8
}

#[lang="exchange_free"]
unsafe fn deallocate(ptr: *mut u8, _size: uint, _align: uint) {
  // libc::free(ptr as *mut libc::c_void)
}
