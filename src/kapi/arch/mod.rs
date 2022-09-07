pub mod chipset;
pub mod cpu;
pub mod family;

mod interface_impl;

use crate::hal::ArchInterfaces;

pub const fn get_arch_interfaces() -> ArchInterfaces {
    interface_impl::x86::get_arch_interfaces()
}
