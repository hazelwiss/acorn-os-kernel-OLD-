#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(const_maybe_uninit_zeroed)]
#![allow(dead_code)]
#![feature(bench_black_box)]

#[macro_use]
extern crate klib;

use klib::{drivers, kapi, mem, shell};

#[no_mangle]
pub unsafe extern "C" fn kmain() -> ! {
    kapi::init();
    mem::init();
    drivers::console::clear();
    loginf!("booting kernel!");
    logok!("drivers initialized.");
    drivers::irq::enable();
    logok!("interrupts enabled.");
    loginf!("starting shell.");
    shell::run();
    logerr!("hit kernel endpoint! Halting...");
    loop {}
}
