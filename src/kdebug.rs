#![allow(dead_code)]
extern crate core;
use core::prelude::*;
use core::fmt::*; use mem;
use libc;

pub static	N_GSYM		: u8 = 0x20;	// global symbol
pub static	N_FNAME		: u8 = 0x22;	// F77 function name
pub static	N_FUN		: u8 = 0x24;	// procedure name
pub static	N_STSYM		: u8 = 0x26;	// data segment variable
pub static	N_LCSYM		: u8 = 0x28;	// bss segment variable
pub static	N_MAIN		: u8 = 0x2a;	// main function name
pub static	N_PC		: u8 = 0x30;	// global Pascal symbol
pub static	N_RSYM		: u8 = 0x40;	// register variable
pub static	N_SLINE		: u8 = 0x44;	// text segment line number
pub static	N_DSLINE	: u8 = 0x46;	// data segment line number
pub static	N_BSLINE	: u8 = 0x48;	// bss segment line number
pub static	N_SSYM		: u8 = 0x60;	// structure/union element
pub static	N_SO		: u8 = 0x64;	// main source file name
pub static	N_LSYM		: u8 = 0x80;	// stack variable
pub static	N_BINCL		: u8 = 0x82;	// include file beginning
pub static	N_SOL		: u8 = 0x84;	// included source file name
pub static	N_PSYM		: u8 = 0xa0;	// parameter variable
pub static	N_EINCL		: u8 = 0xa2;	// include file end
pub static	N_ENTRY		: u8 = 0xa4;	// alternate entry point
pub static	N_LBRAC		: u8 = 0xc0;	// left bracket
pub static	N_EXCL		: u8 = 0xc2;	// deleted include file
pub static	N_RBRAC		: u8 = 0xe0;	// right bracket
pub static	N_BCOMM		: u8 = 0xe2;	// begin common
pub static	N_ECOMM		: u8 = 0xe4;	// end common
pub static	N_ECOML		: u8 = 0xe8;	// end common (local name)
pub static	N_LENG		: u8 = 0xfe;	// length of preceding entry

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

fn stab_binsearch(stabs: &[Stab], left: &mut u32, right: &mut u32, ty: u8, addr: u32) {
    let mut l = *left;
    let mut r = *right;
    let mut any_matches = false;
    let mut ad = addr;

    while l <= r {
        let true_m = (l + r)/2;
        let mut m = true_m;

        cprintf!("stabs:len %d  %p l: %d r: %d m: %d\n", stabs.len(), stabs.as_ptr(), l, r, m);
        stabs[0].n_type == 1;
        while m >=(l as u32) && stabs[m as uint].n_type != ty { 
            if m == 0 { break; } // fixed bug :|
            m = m - 1;
        }
        if m < l { 
            l  = true_m + 1;
            continue;
        }

        any_matches = true;
        if stabs[m as uint].n_value < ad {
            *left = m;
            l = true_m + 1;
        } else if stabs[m as uint ].n_value > ad {
            *right = m - 1;
            r = m - 1;
        } else {
            *left = m;
            l = m;
            ad = ad + 4;
        }
    }

    if !any_matches { *right = *left - 1; }
    else { 
        l = *right;
        while l > *left && stabs[l as uint ].n_type != ty { l = l - 1; }
        *left = l;
    }
}

pub fn debuginfo_eip(addr: u32)-> Eipdebuginfo {

    let mut addr = addr;
    let mut info = Eipdebuginfo {
                    eip_file : "<unknown>",
                    eip_line : 0,
                    eip_fn_name : "<unknown>",
                    eip_fn_addr : addr,
                    eip_fn_narg : 0
                };

    if addr <= 0 /* ULIM */ {
        cprintf!("User Address\n");
        return info;
    }

    let stabs: u32 = libc::origin::__STAB_BEGIN__ as u32;
    let stab_end: u32 = libc::origin::__STAB_END__ as u32;
    let stabstr: u32 = libc::origin::__STABSTR_BEGIN__ as u32;
    let stabstr_end: u32 = libc::origin::__STABSTR_END__ as u32;
    if stabstr > stabstr_end { return info;}

    let size = unsafe { (core::intrinsics::size_of::<Stab>() as u32) };
    let mut lfile = 0u32;
    let mut rfile: u32 = (stab_end - stabs) / size - 1;
    let newstabs: &[Stab] = unsafe { core::mem::transmute( (stabs, rfile + 1) ) } ;
    stab_binsearch(newstabs, &mut lfile, &mut rfile, N_SO, addr);
    if lfile == 0 { return info;}

    let mut lfun = lfile;
    let mut rfun = rfile;
    let mut lline: u32;
    let mut rline: u32;
    stab_binsearch(newstabs, &mut lfun, &mut rfun, N_FUN, addr);

    if lfun <= rfun {
        if newstabs[lfun as uint ].n_strx < stabstr_end - stabstr {
            // info.eip_fn_name = stabstr + newstabs[lfun as uint].n_strx;
        }
        info.eip_fn_addr = newstabs[lfun as uint].n_value;
        addr  = addr - info.eip_fn_addr;
        lline = lfun;
        rline = rfun;
    } else {
        info.eip_fn_addr = addr;
        lline = lfile;
        rline = rfile;
    }

    if lline > rline { return info;}
    stab_binsearch(newstabs, &mut lline, &mut rline, N_SLINE, addr);
    if lline <= rline {
        info.eip_line = rline as int;
    }

    // info.eip_
    //
    while lline >= lfile 
        && newstabs[lline as uint ].n_type != N_SOL
            && (newstabs[lline as uint ].n_type != N_SO || newstabs[lline as uint].n_value != 0)
            { lline = lline - 1; }
    if lline >= lfile && newstabs[lline as uint ].n_strx < (stabstr_end - stabstr)/size
    { 
        // info.eip_file = stabstr + newstabs[lline as uint].n_strx; 
    }


    if lfun < rfun {
        while lline < rfun && newstabs[lline as uint].n_type == N_PSYM {
            info.eip_fn_narg += 1;
            lline = lline + 1; }
    }

    info
}


