//! wrapper of C functions
//!

extern crate core;

pub mod origin {
    extern "C" {
        pub fn heapbase() -> uint;
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
    use macros;

    pub fn to_cstring(buf: &mut [u8], s:& str) {
        if s.len() >= buf.len() { return ; }
        for i in range(0, s.len()) {
            buf[i] = s.as_bytes()[i];
        }
        buf[s.len()] = 0;
    }

    pub fn readline(promt: &str) -> *const u8 {
        let mut buf = [0u8, ..512];
        to_cstring(buf, promt);
        unsafe { super::origin::readline(buf.as_ptr()) }
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

