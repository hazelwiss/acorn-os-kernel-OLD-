#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![allow(dead_code)]

#[macro_use]
extern crate alloc;
extern crate kapi;

#[macro_use]
mod log;
mod mem;
mod panic;
mod shell;

use kapi::{arch, drivers, hal, util};

#[no_mangle]
pub unsafe extern "C" fn kmain() -> ! {
    drivers::init();
    hal::console::clear();
    loginf!("booting kernel!");
    logok!("drivers initialized.");
    loginf!("Acorn OS");
    loginf!("starting shell.");
    shell::run();
    logerr!("hit kernel endpoint! Halting...");
    loop {}
}
