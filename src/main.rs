#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![allow(dead_code)]

#[macro_use]
extern crate alloc;
extern crate kapi;
extern crate kutil;

#[macro_use]
mod log;
mod mem;
mod panic;
mod shell;
mod symbols;

use kapi::{arch, drivers, hal};

#[no_mangle]
pub unsafe extern "C" fn kmain() -> ! {
    drivers::init();
    mem::init();
    hal::console::clear();
    loginf!("booting kernel!");
    logok!("drivers initialized.");
    loginf!("Acorn OS");
    loginf!("starting shell.");
    //let _string_crash = alloc::string::String::with_capacity(1000000000);
    shell::run();
    logerr!("hit kernel endpoint! Halting...");
    loop {}
}
