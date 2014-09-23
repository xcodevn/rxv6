#![crate_type="staticlib"]
#![no_std]
#![feature(globs)]
#![feature(lang_items)]

#![feature(asm)]
#![allow(ctypes)]
#![allow(while_true)]
#![allow(unused_imports)] 
#![allow(unused_variable)]

extern crate core;

use core::prelude::*;

pub mod tools;
pub mod runtime;
pub mod console;
pub mod libc;

#[no_mangle]
pub fn main() {
    libc::console::init();
    console::run();
}
