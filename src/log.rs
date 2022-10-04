#![allow(unused)]

use core::fmt::{self, Write};
use drivers::serial::debug;

struct WriterSerial;

impl fmt::Write for WriterSerial {
    #[inline]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        debug::puts(s);
        Ok(())
    }
}

pub fn log_serial(args: fmt::Arguments) -> fmt::Result {
    WriterSerial {}.write_fmt(args)
}
