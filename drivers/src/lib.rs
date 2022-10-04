//! Device drivers for communicating with hardware.

#![no_std]
#![warn(missing_docs)]

#[allow(unused)]
#[macro_use]
extern crate platform;
extern crate proc_macros_drivers as proc_macros;

pub mod serial;

/// Initializes all drivers.
pub fn init() {}
