#![crate_type="staticlib"]
#![no_std]
#![feature(globs, lang_items, macro_rules, asm)]
#![allow(ctypes, while_true, unused_imports, unused_variable)]

extern crate core;

use core::prelude::*;

pub mod macros;
pub mod tools;
pub mod runtime;
pub mod console;
pub mod libc;

#[no_mangle]
pub fn main() {
    libc::console::init();
    console::run();
}

