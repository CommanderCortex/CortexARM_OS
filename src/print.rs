#[macro_export]
macro_rules! cprint {
    ($($arg:tt)*) => {
        use core::fmt::Write;
        let mut console = crate::serialdriver::SerialDriver::new(0x0900_0000 as *mut u8);
        let _ = console.write_fmt(format_args!($($arg)*));
    }
}