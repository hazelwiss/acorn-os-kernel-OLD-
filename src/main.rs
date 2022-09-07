#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(const_maybe_uninit_zeroed)]
#![allow(dead_code)]
#![feature(bench_black_box)]

#[macro_use]
extern crate alloc;

#[macro_use]
mod log;
mod kapi;
mod kutil;
mod mem;
mod panic;
mod shell;
mod symbols;

use kapi::{arch, drivers, hal};

#[no_mangle]
pub unsafe extern "C" fn kmain() -> ! {
    kapi::init();
    mem::init();
    hal::console::clear();
    loginf!("booting kernel!");
    logok!("drivers initialized.");
    loginf!("Acorn OS");
    loginf!("enabling interrupts.");
    hal::irq::geirq();
    loginf!("starting shell.");
    shell::run();
    logerr!("hit kernel endpoint! Halting...");
    loop {}
}
