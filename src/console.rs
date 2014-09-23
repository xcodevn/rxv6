extern crate core;
use core::prelude::*;
use core;
use core::fmt::*;

use libc;
use libc::console::{print, println};
use libc::console;

use macros;

pub fn helloworld() {
    for i in range(0, 10i) {
        console::set_textcolor(i);
        println("Hello world")
    }

    let x = 5i;
    let c: int;
    let (a, b) = (1i, 2i);
    
    // let mut string = "Hello there".to_string();
    let mut nums = [1i, 2i, 3i];
    nums[1] = 0;
}


pub fn run () {
    console::set_textcolor(9);
    cprintf!("RXV6 loaded!\n");
    console::set_textcolor(7);
    cprintf!("Type 'help' for command list.\n");

    loop {
        let st = console::readline("# ");
        // call hello world ;)
        helloworld()
    }
}
