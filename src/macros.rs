#![macro_escape]

/// All useful macros in our kernel usecase

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

#[macro_export]
macro_rules! cstr(
    ($ss:expr) =>({ let mut buf = [0u8, ..512]; libc::console::to_cstring(buf, $ss); buf })
)

#[macro_export]
macro_rules! sprintf(
    ($buf:expr, $fmt:expr $(,$var:expr)*) => (
        unsafe{ libc::origin::snprintf($buf.as_mut_ptr(), $buf.len() as int, cstr!($fmt).as_ptr() $(,$var)*); }
    )
)

#[macro_export]
macro_rules! try(
    ($e:expr) => (match $e { Ok(e) => e, Err(e) => return Err(e) })
)
