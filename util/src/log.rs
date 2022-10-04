#![allow(unused)]

use core::fmt::{self, Write};

pub type LogFn = fn(args: fmt::Arguments) -> fmt::Result;

struct Writer {
    log: LogFn,
}

/// Log function which does nothing with the input.
pub fn log_drain(_args: fmt::Arguments) -> fmt::Result {
    Ok(())
}

static mut WRITER: Writer = Writer { log: log_drain };

pub fn log(args: fmt::Arguments) -> fmt::Result {
    unsafe { (WRITER.log)(args) }
}

pub fn set_log_fn(f: LogFn) {
    unsafe {
        WRITER.log = f;
    }
}

#[macro_export]
macro_rules! log {
    ($txt:literal) => {
        {
            ::util::log::log(format_args!($txt)).expect("formatting error")
        }
    };
    ($txt:literal,$($args:expr),*$(,)?) => {
        {
            ::util::log::log(format_args!($txt,$($args),*)).expect("formatting error");
        }
    };
}

#[macro_export]
macro_rules! logln {
    ($($ts:tt)*) => {
        log!("{}\n\r", format_args!($($ts)*));
    };
}

#[macro_export]
macro_rules! logi {
    ($($ts:tt)*) => {
        logln!("[INFO] {}", format_args!($($ts)*))
    };
}

#[macro_export]
macro_rules! logw {
    ($($ts:tt)*) => {
        logln!("[WARN] {}", format_args!($($ts)*));
    };
}

#[macro_export]
macro_rules! loge {
    ($($ts:tt)*) => {
        logln!("[ERROR] {}", format_args!($($ts)*));
    };
}
