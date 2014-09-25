

#[cfg(target_arch = "x86")]
pub fn read_ebp() -> u32 {
    let mut c: u32;
    unsafe { 
        asm!("movl %ebp, $0"
                  : "=r"(c)
                 ); 
    }

    c
}
