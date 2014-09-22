
// Our tools for kernel use

#[inline(always)]
pub fn out<T>(port: u16, val: T) {
    unsafe {
        asm!("out $1, $0" :: "{al}"(val), "{dx}"(port) :: "intel");
    }
}

#[inline(always)]
pub fn inb(port: u16) -> u8 {
    let mut val: u8;
    unsafe {
        asm!("in $0, $1" : "={al}"(val) : "{dx}"(port) :: "intel");
    }
    val
}

pub fn wait(port: u16, mask: u8) {
    while inb(port) & mask != 0 {}
}
