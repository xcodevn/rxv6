/// Basic interactive mechamisn for kernel dev
/// Support simple commands
///
extern crate core;
use core::fmt::*;
use core::prelude::*;

use libc;
use mem;
use asm;
use libc::console::{print, println};
use libc::console;
use backtrace;

use macros;

struct Cmd {
    name: &'static str,
    info: &'static str,
    action: fn()
}

/// Simple Writer for demangel C++ style purpose
pub struct B512Writer {
    buf: [u8, ..512],
    pos: uint,
}

impl B512Writer {
    /// Return raw String!
    pub fn as_ptr<'a>(self: &'a B512Writer) -> *const u8 {
        self.buf.as_ptr()
    }

    /// Return new instance
    pub fn new() -> B512Writer {
        B512Writer {
            buf: [0u8, ..512],
            pos: 0,
        }
    }

    /// Write str into the buffer
    pub fn write_str(&mut self, bytes: &str) -> Result< (), core::fmt::FormatError >  {
        if bytes.len() + self.pos > 512 { return Err( WriteError ) }
        for i in range(0, bytes.len()) { self.buf[self.pos + i] = bytes.as_bytes()[i] }
        self.pos += bytes.len();
        Ok(())
    }
}


/// Support function for char type
pub trait CharFns {
    fn is_digit(self: Self) -> bool ;
}

impl CharFns for char {
    /// is in set ['0'..'9']
    fn is_digit(self: char) -> bool {
        self >= '0' && self <= '9'
    }
}

impl Cmd {
    /// Call the action function which save in the struct
    fn action(&self) {
        let call = self.action;
        call()
    }
}

/// Print out list of current supported commands
fn mon_help() {

    cprintf!("\nCommand list\n");
    for i in range(0, cmds.len()) {
        cprintf!("\t%s\n", cmds[i].info.as_ptr());
    }

    cprintf!("\n");
}

/// Show the stacktrace by using `backtrace` lib
/// ported from Rust source code
fn mon_backtrace () {
    cprintf!("Call stack:\n");
    let mut ebp: u32 = asm::read_ebp();
    let mut count = 0i;
    loop {
        let eip = unsafe {*((ebp + 4) as *const u32)};
        let mut dbinfo: (&str, &str, u32) = ("<unk>", "<unk>", 0u32);
        unsafe { libc::origin::fileline_debug(eip, &mut dbinfo) ; }
        let (filename, funcname, lineno) = dbinfo;

        let mut writer = B512Writer::new();

        let rl = backtrace::demangle(&mut writer, funcname);

        cprintf!("[%2d]  eip: %08p function %s at %s:%d\n", count, eip, writer.as_ptr(), filename.as_ptr(), lineno);
        ebp = unsafe { *(ebp as *const u32) };
        if eip == 0 { break; }
        count += 1;
    }
}

/// Useful information about kernel
fn mon_kerninfo () {
    cprintf!("MEM SIZE %d MB\n", mem::get_memsize()/1024/1024 + 1);
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

/// List cmd struct
static cmds : [Cmd,..3] = [ 
    Cmd { name: "help",      info: "help      :  getting this help message\x00", action: mon_help} ,
    Cmd { name: "kerninfo",  info: "kerninfo  :  kernel memory region info \x00", action: mon_kerninfo} ,
    Cmd { name: "backtrace", info: "backtrace :  show backtrace stack\x00", action: mon_backtrace} ,
];


/// Main function of montitor
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
