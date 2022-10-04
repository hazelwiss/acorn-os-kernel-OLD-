//! The x86 family of architectures.

pub mod io;
pub mod pic;

use core::arch::asm;

/// Halts the CPU.
pub fn halt() {
    unsafe { asm!("hlt") }
}

/// Clear interrupt.
/// Prevents interrupts from firing.
pub fn cli() {
    unsafe { asm!("cli") }
}

/// Set interrupt.
/// Allows interrupts to fire.
pub fn sti() {
    unsafe { asm!("sti") }
}
