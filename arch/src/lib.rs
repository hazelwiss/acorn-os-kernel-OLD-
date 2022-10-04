//! Architecture specific code pertaining to both the CPU and chipset.

#![no_std]
#![allow(dead_code)]
#![warn(missing_docs)]
#![feature(const_maybe_uninit_zeroed)]

pub extern crate proc_macros_arch as proc_macros;

extern crate self as arch;

pub mod cpu;
pub mod fam;
