//! wrapper of C functions
//!

extern crate core;

pub mod origin {
    extern "C" {
        /* symbols addr */
        pub fn _start();
        pub fn entry();
        pub fn etext();
        pub fn edata();
        pub fn end();
        pub fn malloc(size: u32) -> *mut u8;
        pub fn free(ptr: *const u8);
        pub fn bootheap();
        pub fn __STAB_BEGIN__();
        pub fn __STAB_END__();
        pub fn __STABSTR_BEGIN__();
        pub fn __STABSTR_END__();

        pub fn snprintf(buf: *mut u8, n: int, fmt: *const u8, ...);
        pub fn cprintf(fmt: *const u8, ...);
        pub fn cons_init();
        pub fn readline(promt: *const u8) -> *const u8;
        pub fn set_bgcolor(color: int);
        pub fn set_textcolor(color: int);
    }
}


pub mod console {
    use core::prelude::*;
    use core::mem;
    use macros;

    pub fn to_cstring(buf: &mut [u8], s:& str) {
        if s.len() >= buf.len() { return ; }
        for i in range(0, s.len()) {
            buf[i] = s.as_bytes()[i];
        }
        buf[s.len()] = 0;
    }

    pub fn to_str(ptr: *const u8)-> &'static str {
        let v: &[u8] = unsafe { mem::transmute( (ptr, 256u) ) };
        let mut c = 0u;
        while v[c] != 0 { c = c + 1; }
        unsafe { mem::transmute( (ptr, c) ) }
    }

    pub fn readline(promt: &str) -> &str {
        let mut buf = [0u8, ..512];
        to_cstring(buf, promt);
        let ptr = unsafe { super::origin::readline(buf.as_ptr()) };
        to_str(ptr)
    }

    pub fn print(msg: &str) {
        let mut buf = [0u8, ..512];
        to_cstring(buf, msg);
        unsafe { super::origin::cprintf(buf.as_ptr()); }
    }

    pub fn println(msg: &str) {
        let mut buf = [0u8, ..512];
        to_cstring(buf, msg);
        unsafe { super::origin::cprintf(buf.as_ptr()); }
        print("\n\x00");
    }

    pub fn init() {
        unsafe { super::origin::cons_init(); }
    }

    pub fn set_bgcolor(color: int) {
        unsafe { super::origin::set_bgcolor(color); }
    }

    pub fn set_textcolor(color: int) {
        unsafe { super::origin::set_textcolor(color); }
    }
}

