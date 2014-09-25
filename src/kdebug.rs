extern crate core;
use core::prelude::*;
use core;
use core::fmt::*; use mem;
use libc;

pub static	N_GSYM		: uint = 0x20;	// global symbol
pub static	N_FNAME		: uint = 0x22;	// F77 function name
pub static	N_FUN		: uint = 0x24;	// procedure name
pub static	N_STSYM		: uint = 0x26;	// data segment variable
pub static	N_LCSYM		: uint = 0x28;	// bss segment variable
pub static	N_MAIN		: uint = 0x2a;	// main function name
pub static	N_PC		: uint = 0x30;	// global Pascal symbol
pub static	N_RSYM		: uint = 0x40;	// register variable
pub static	N_SLINE		: uint = 0x44;	// text segment line number
pub static	N_DSLINE	: uint = 0x46;	// data segment line number
pub static	N_BSLINE	: uint = 0x48;	// bss segment line number
pub static	N_SSYM		: uint = 0x60;	// structure/union element
pub static	N_SO		: uint = 0x64;	// main source file name
pub static	N_LSYM		: uint = 0x80;	// stack variable
pub static	N_BINCL		: uint = 0x82;	// include file beginning
pub static	N_SOL		: uint = 0x84;	// included source file name
pub static	N_PSYM		: uint = 0xa0;	// parameter variable
pub static	N_EINCL		: uint = 0xa2;	// include file end
pub static	N_ENTRY		: uint = 0xa4;	// alternate entry point
pub static	N_LBRAC		: uint = 0xc0;	// left bracket
pub static	N_EXCL		: uint = 0xc2;	// deleted include file
pub static	N_RBRAC		: uint = 0xe0;	// right bracket
pub static	N_BCOMM		: uint = 0xe2;	// begin common
pub static	N_ECOMM		: uint = 0xe4;	// end common
pub static	N_ECOML		: uint = 0xe8;	// end common (local name)
pub static	N_LENG		: uint = 0xfe;	// length of preceding entry

// Entries in the STABS table are formatted as follows.
pub struct Stab {
	n_strx: u32,    // index into string table of name
	n_type: u8,     // type of symbol
	n_other: u8,    // misc info (usually empty)
	n_desc: u16,    // description field
	n_value: u32,   // value of symbol
}

// kernel debug info
//

// store function debug information
pub struct Eipdebuginfo {
    eip_file: &'static str,
    eip_line: int,
    eip_fn_name: &'static str,
    eip_fn_addr: u32,
    eip_fn_narg: int
}

fn stab_binsearch(stabs: &[Stab], left: &mut u32, right: &mut u32, ty: uint, addr: u32) {
}

pub fn debuginfo_eip(addr: u32, info: &mut Eipdebuginfo) {

    info.eip_file = "<unknown>";
    info.eip_line = 0;
    info.eip_fn_name = "<unknown>";
    info.eip_fn_addr = addr;
    info.eip_fn_narg = 0;

    if addr <= 0 /* ULIM */ {
        cprintf!("USER ADDRESS\n");
        1/0u;
    }

    let stabs: u32 = libc::origin::__STAB_BEGIN__ as u32;
    let stab_end: u32 = libc::origin::__STAB_END__ as u32;
    let stabstr: u32 = libc::origin::__STABSTR_BEGIN__ as u32;
    let stabstr_end: u32 = libc::origin::__STABSTR_END__ as u32;
    if stabstr > stabstr_end { return }

    let mut lfile = 0u32;
    let mut rfile: u32 = stab_end - stabs;
    let num = rfile / unsafe { (core::intrinsics::size_of::<Stab>() as u32) };
    let newstabs: &[Stab] = unsafe { core::mem::transmute( (stabs, num) ) } ;
    stab_binsearch(newstabs, &mut lfile, &mut rfile, N_SO, addr);
    if lfile == 0 { return }
}


