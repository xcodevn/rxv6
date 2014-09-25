
pub static KERNBASE :u32 = 0xF0000000;

pub fn roundup(addr:u32, up: u32) -> u32 {
    if addr % up == 0 { addr } else { addr - (addr % up) + up } 
}
