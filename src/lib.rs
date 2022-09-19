#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(const_maybe_uninit_zeroed)]
#![allow(dead_code)]
#![feature(bench_black_box)]

#[macro_use]
extern crate alloc;
extern crate self as klib;

#[macro_use]
pub mod log;
pub mod arch;
pub mod drivers;
pub mod fs;
pub mod hal;
pub mod irq;
pub mod keyboard;
pub mod kutil;
pub mod mem;
pub mod panic;
pub mod shell;
pub mod symbols;