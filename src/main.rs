#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![allow(dead_code)]

mod arch;
mod drivers;
mod kapi;
#[macro_use]
mod log;
mod hal;
mod panic;
mod shell;
mod util;

#[no_mangle]
pub unsafe extern "C" fn kmain() -> ! {
    kapi::drivers::init();
    loginf!("booting kernel!");
    logok!("drivers initialized.");
    loginf!("Acorn OS");
    logerr!("hit kernel endpoint! Halting...");
    loop {}
}
