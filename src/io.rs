//! IO MODULE
//! Input/Output functions for kernel

/// send :data to :port IO port
#[inline(always)]
pub fn outb (port: u16, data: u8) {
    unsafe { asm!( "outb $0, $1" : : "{al}" (data), "{dx}" (port)); }
}

/// get a byte from IO port :port
#[inline(always)]
pub fn inb (port: u16) -> u8 {
    let mut rl: u8;
    unsafe { asm!( "inb $1, $0" : "={al}"(rl) : "{dx}"(port)); }
    rl
}
