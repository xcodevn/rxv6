#![crate_type="staticlib"]
#![no_std]
#![feature(globs, lang_items, macro_rules, asm)]
#![allow(ctypes, while_true, unused_imports, unused_variable)]

///
/// Main module file
/// Include all other sub-modules
///

extern crate core;

use core::prelude::*;

pub mod macros;
pub mod asm;
pub mod io;
pub mod mem;
pub mod runtime;
pub mod monitor;
pub mod libc;
pub mod backtrace;

#[no_mangle]
/// Init the kernel 
pub fn main() {
    libc::console::init();      // init CGA and Serial port
    monitor::run();             // excute the monitor loop
}
