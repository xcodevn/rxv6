#![crate_type="lib"]
#![no_std]
#![feature(globs)]
#![feature(lang_items)]

#![feature(asm)]
#![allow(ctypes)]
#![allow(while_true)]

extern crate core;

use core::prelude::*;

pub mod tools;
pub mod runtime;
pub mod console;
pub mod libc;

fn console () {
    let mut a = box 0i;
    unsafe {
        libc::cons_init();
        libc::cprintf(" RXV6 - i386 \n\x00".as_ptr());
        while true {
            let st = libc::readline(">> \x00".as_ptr());
            libc::cprintf("[%o]\t%s\n\x00".as_ptr(), *a, st);
            *a = *a + 1;
        }
    }
}

#[no_mangle]
pub fn main() {
    console();
}
