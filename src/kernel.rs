
#![feature(asm)]

pub mod tools;
pub mod runtime;
pub mod console;

fn clear_screen(background: u16) {
    for i in range(0u, 80 * 25) {
        unsafe {
            *((0xb8000 + i * 2) as *mut u16) = background << 12;
        }
    }
}

#[no_mangle]
pub fn main() {
    clear_screen(0); // Yellow
    tools::wait(2,2);
}
