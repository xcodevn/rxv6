#![crate_type="staticlib"]
#![no_std]
#![feature(globs, lang_items, macro_rules, asm)]
#![allow(ctypes, while_true, unused_imports, unused_variable)]

extern crate core;

use core::prelude::*;

pub mod macros;
pub mod tools;
pub mod asm;
pub mod kdebug;
pub mod mem;
pub mod runtime;
pub mod monitor;
pub mod libc;

#[no_mangle]
pub fn main() {
    let a = box 3i;
    libc::console::init();
    monitor::run();
}
