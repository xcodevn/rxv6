extern crate core;
use core::prelude::*;
use core::fmt::*;

use libc;
use mem;
use asm;
use kdebug;
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

fn mon_help() {
    cprintf!("\nCommand list\n");
    for i in range(0, cmds.len()) {
        cprintf!("\t%s\n", cmds[i].info.as_ptr());
    }
    cprintf!("\n");
}

fn mon_backtrack () {
    cprintf!("ebp : %p", asm::read_ebp());
}

fn mon_kerninfo () {
    cprintf!("Special kernel symbols:\n");
    let (_start, entry, etext, edata, end) = { use libc::origin::*;
                                                (_start as u32, 
                                                 entry as u32, etext as u32 , edata as u32, 
                                                 end as u32) };
    cprintf!("  _start                  %08x (phys)\n", _start);
    cprintf!("  entry  %08x (virt)  %08x (phys)\n", entry, entry - mem::KERNBASE);
    cprintf!("  etext  %08x (virt)  %08x (phys)\n", etext, etext - mem::KERNBASE);
    cprintf!("  edata  %08x (virt)  %08x (phys)\n", edata, edata - mem::KERNBASE);
    cprintf!("  end    %08x (virt)  %08x (phys)\n", end, end - mem::KERNBASE);
    cprintf!("Kernel executable memory footprint: %dKB\n",
                     mem::roundup(end - entry, 1024) / 1024);
}

static cmds : [Cmd,..3] = [ 
    Cmd { name: "help",      info: "help      :  getting this help message\x00", action: mon_help} ,
    Cmd { name: "kerninfo",  info: "kerninfo  :  kernel memory region info \x00", action: mon_kerninfo} ,
    Cmd { name: "backtrack", info: "backtrack :  show backtrack stack\x00", action: mon_backtrack} , 
];

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
