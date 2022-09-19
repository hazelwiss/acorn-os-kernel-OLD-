#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(const_maybe_uninit_zeroed)]
#![allow(dead_code)]
#![feature(bench_black_box)]

#[macro_use]
extern crate klib;

use klib::{drivers, hal, irq, mem, shell};

#[no_mangle]
pub unsafe extern "C" fn kmain() -> ! {
    hal::init();
    drivers::init();
    mem::init();
    drivers::console::clear();
    loginf!("booting kernel!");
    logok!("drivers initialized.");
    irq::enable();
    logok!("interrupts enabled.");
    loginf!("starting shell.");
    shell::run();
    logerr!("hit kernel endpoint! Halting...");
    loop {}
}
