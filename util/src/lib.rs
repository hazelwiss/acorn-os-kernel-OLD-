#![no_std]
#![feature(bench_black_box)]

pub extern crate proc_macros_util as proc_macros;
pub extern crate spin;

pub mod log;

#[inline]
pub fn delay(time: u64) {
    ::core::hint::black_box(for _ in 0..time {});
}
