use core::fmt::{self, Write};

use klib::drivers::serial_out;

struct Writer {}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        serial_out::puts(s);
        Ok(())
    }
}

#[panic_handler]
unsafe fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    let location = info.location().unwrap_unchecked();
    let msg = info.message().unwrap_unchecked();
    Writer {}
        .write_fmt(format_args!("[PANIC] {location}\n{msg}\n"))
        .unwrap_unchecked();
    loop {}
}
