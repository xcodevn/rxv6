extern crate core;
use core::prelude::*;
use core::fmt::*;

use libc;
use libc::console::{print, println};
use libc::console;

use macros;

struct Cmd {
    name: &'static str,
    info: &'static str,
    action: fn()
}

impl Cmd {
    fn action(&self) {
        let call = self.action;
        call()
    }
}

fn help() {
    cprintf!("\nCommand list\n");
    for i in range(0, cmds.len()) {
        cprintf!("\t%s\n", cmds[i].info.as_ptr());
    }
    cprintf!("\n");
}

fn backtrack () {
    cprintf!("\nbacktrack\n");
}

static cmds : &'static [Cmd] = [ Cmd { name: "help",      info: "help      :  getting this help message\x00", action: help} ,
                                 Cmd { name: "backtrack", info: "backtrack :  show backtrack stack\x00", action: backtrack} , ];

pub fn run () {

    console::set_textcolor(9);
    cprintf!("RXV6 loaded!\n");
    console::set_textcolor(7);
    cprintf!("Type 'help' for command list.\n");

    loop {
        let st = console::readline("# ");
        for i in range(0, cmds.len() ) {
            if st == cmds[i].name { cmds[i].action(); }
        }
    }
}
