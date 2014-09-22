
#![feature(asm)]

extern crate libc;


pub mod tools;
pub mod runtime;
pub mod console;
pub mod libc;


fn console () {
    let mut a = 0i;
    unsafe {
        libc::cons_init();
        libc::cprintf(" RXV6 - i386 \n\x00".as_ptr());
        while (true) {
            let st = libc::readline(">> \x00".as_ptr());
            libc::cprintf("[%d]     %s\n\x00".as_ptr(), a, st);
            a = a + 1;
        }
    }
}

#[no_mangle]
pub fn main() {
    console();
}
