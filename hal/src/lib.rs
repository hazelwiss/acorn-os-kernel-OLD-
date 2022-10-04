//! HAL - Hardware Abstaction Layer
//! Exposes types and methods intended to be used by the kernel
//! and device drivers to create platform agnostic implementations.
//! This crate itself doesn't implement the actual HAL itself, but
//! just exposes its interface to the rest of the codebase.

#![no_std]
#![warn(missing_docs)]

extern crate proc_macros_hal as proc_macros;
extern crate self as hal;

pub mod fb;
pub mod irq;
pub mod kbd;
pub mod serial;

/// Creates a HAL interface.
pub use proc_macros::interface;

/// HAL interface.
pub trait IHAL {}
