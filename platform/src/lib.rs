//! Implements both the architecure specific and hardware specific code
//! to use the HAL interface. This is the crate used by the kernel and
//! device drivers to achieve platform agnostic code.

#![no_std]
#![warn(missing_docs)]
#![feature(unboxed_closures)]
#![feature(trait_alias)]

extern crate proc_macros_platform as proc_macros;
extern crate self as platform;
#[allow(unused)]
#[macro_use]
extern crate util;

pub mod components;
pub mod types;
pub mod utils;

mod platforms;

use core::marker::PhantomData;

/// A wrapper around a requested component.
#[must_use]
pub enum Requested<T> {
    /// The requested component was found.
    Requested(T),
    /// The requested component is unavaible.
    Unavailable,
    /// The requested component is unsupported
    /// for the current platform.
    Unsupported,
}

#[doc(hidden)]
pub trait Interface<CPU: platforms::Arch, CS: platforms::Chipset>: Sized {
    fn init();
    fn on_request() -> Requested<Self>;
}

#[doc(hidden)]
pub struct Platform<CPU: platforms::Arch, Chipset: platforms::Chipset> {
    _p: PhantomData<(CPU, Chipset)>,
}

impl<CPU: platforms::Arch, CS: platforms::Chipset> Platform<CPU, CS> {
    #[inline]
    #[doc(hidden)]
    pub fn request<C: Interface<CPU, CS>>() -> Requested<C> {
        C::on_request()
    }

    #[inline]
    #[doc(hidden)]
    pub fn request_map<C: Interface<CPU, CS>>(mut f: impl FnMut(&mut C)) {
        let c = C::on_request();
        match c {
            Requested::Requested(mut req) => f(&mut req),
            _ => {}
        }
    }
}

use platforms::NativeArch;
use platforms::NativeCS;
#[doc(hidden)]
pub use platforms::NativePlatform;

/// Initializes the platform.
pub fn init() {
    components::init();
    platforms::init::<NativePlatform>();
}
