#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![allow(dead_code)]

#[macro_use]
extern crate util;
#[allow(unused)]
#[macro_use]
extern crate alloc;
extern crate self as kernel;

mod boot;
mod exceptions;
mod interrupts;
mod log;
mod mem;
mod panic;
mod syms;

static VER_MAJOR_STR: &'static str = "0";
static VER_MINOR_STR: &'static str = "0";
static VER_PATCH_STR: &'static str = "1";

boot::entry_point!(kmain);

fn kinit() {
    platform::init(platform::conf::PlatformConfig {
        exception_clbks: platform::conf::ExceptionClbks {
            pg_fault: exceptions::pg_fault,
            arch_specific: exceptions::arch_specific,
        },
        interrupts_clbks: platform::conf::InterruptClbks {
            kbd_input: interrupts::kbd_interrupt,
            arch_specific: interrupts::arch_specific,
        },
    });
    // Sets the logging function.
    util::log::set_log_fn(log::log_serial);
    logi!("platform initialized.");
    drivers::init();
    mem::init();
}

fn kmain(_info: &'static mut boot::BootInfo) -> ! {
    kinit();
    logi!("Acorn OS v. {VER_MAJOR_STR}.{VER_MINOR_STR}.{VER_PATCH_STR}");
    loop {}
}
