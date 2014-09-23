
extern crate core;
use core::prelude::*;

use libc;
use libc::console::{print, println};
use libc::console;


pub fn run () {
    console::init();
    console::set_textcolor(9);
    println("RXV6 loaded!\x00");
    console::set_textcolor(7);
    println("Type 'help' for getting command info\x00");
    let i = box 3i;
    unsafe {
        while true {
            let j = box *i;
            let st = libc::origin::readline("# \x00".as_ptr());

        }
    }
}
