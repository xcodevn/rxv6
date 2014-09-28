/// Memory management module
/// Mange physical and vitual memory space
///
use io;

pub static KERNBASE :u32 = 0xF0000000; /// Kernel memory base

pub fn roundup(addr:u32, up: u32) -> u32 {
    if addr % up == 0 { addr } else { addr - (addr % up) + up } 
}

/// Return size of high-address memory in byte (exclused the fisst 1MB memory)
pub fn get_memsize() -> u32 {
    io::outb(0x70, 0x30); let lsb = io::inb(0x71);
    io::outb(0x70, 0x31); let msb = io::inb(0x71);
    (lsb as u32| (msb as u32<< 8)) * 1024   // return in-byte value
}
