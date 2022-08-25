use crate::{drivers, shell, util::once};
use core::fmt;
use spin::Mutex;

struct Writer;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        shell::puts(s);
        drivers::serial_out::puts(s);
        Ok(())
    }
}

static WRITER: Mutex<Writer> = Mutex::new(Writer {});

pub fn log_fmt(args: fmt::Arguments) {
    use fmt::Write;
    WRITER
        .lock()
        .write_fmt(args)
        .expect("Unabel to write formatted args");
}

/// Normal logging
macro_rules! log {
    ($fmt:expr) => {
        crate::log::log_fmt(format_args!($fmt))
    };
    ($fmt:expr, $($args:expr)*) => {
        crate::log::log_fmt(format_args!($fmt, $($args),*))
    };
}

/// Info logging
macro_rules! loginf {
    ($fmt:expr) => {
        loginf!($fmt,)
    };
    ($fmt:expr, $($args:expr)*) => {{
        log!("[INFO] ");
        log!($fmt, $($args),*);
        log!("\n");
    }};
}

/// Log on success
macro_rules! logok {
    ($fmt:expr) => {
        logok!($fmt,)
    };
    ($fmt:expr, $($args:expr)*) => {{
        log!("[OK] ");
        log!($fmt, $($args),*);
        log!("\n");
    }};
}

/// Log on error
macro_rules! logerr {
    ($fmt:expr) => {
        logerr!($fmt,);
    };
    ($fmt:expr, $($args:expr)*) => {{
        log!("[ERR] ");
        log!($fmt, $($args),*);
        log!("\n");
    }};
}

pub fn init() {
    once!(
        logok!("Initialized logging!");
    );
}
