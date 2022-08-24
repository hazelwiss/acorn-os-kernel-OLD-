use crate::kapi::{self, drivers::SHELL};
use core::{fmt, sync::atomic::Ordering};
use spin::Mutex;

struct Writer;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match SHELL.console {
            Some(ref co) => (co.puts)(s),
            None => panic!("Invalid shell interface!"),
        }
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
    assert!(kapi::drivers::INITIALIZED.load(Ordering::Relaxed));
    logok!("Initialized logging!");
}
