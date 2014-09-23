#![macro_escape]

#[macro_export]
macro_rules! cprintf(
    ($fmt:expr $(,$var:expr)*) => (
        {
            let mut buf = [0u8, ..512];
            libc::console::to_cstring(buf, $fmt);
            unsafe { libc::origin::cprintf(buf.as_ptr(), $($var),* ); }
        }
    );
)

