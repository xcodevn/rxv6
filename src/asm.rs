/// Assembler low level ops

#[cfg(target_arch = "x86")]
/// Return current %ebp register value
pub fn read_ebp() -> u32 {
    let mut c: u32;
    unsafe { 
        asm!("movl %ebp, $0"
                  : "=r"(c)
                 ); 
    }

    c
}
