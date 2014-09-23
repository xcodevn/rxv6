//! wrapper of C functions
//!

extern crate core;

pub mod origin {
    extern "C" {
        pub fn heapbase() -> uint;
        pub fn snprintf(buf: *mut u8, n: int, fmt: *const u8, ...);
        pub fn cprintf(fmt: *const u8, ...);
        pub fn cons_init();
        pub fn readline(promt: *const u8) -> *mut u8;
        pub fn set_bgcolor(color: int);
        pub fn set_textcolor(color: int);

    }
}

pub mod console {
    use core::prelude::*;

    pub fn print(msg: & str) {
        unsafe { super::origin::cprintf(msg.as_ptr()); }
    }

    pub fn println(msg: & str) {
        unsafe { super::origin::cprintf(msg.as_ptr()); }
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

pub mod string {
}
