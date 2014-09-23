// wrapper of C functions
//
extern "C" {
    pub fn snprintf(buf: *mut u8, n: int, fmt: *const u8, ...);
    pub fn cprintf(fmt: *const u8, ...);
    pub fn cons_init();
    pub fn readline(promt: *const u8) -> *mut u8;
}
