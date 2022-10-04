//! Utility functions.
use crate::{platforms::Arch, Interface, NativeArch, NativeCS, NativePlatform, Requested};

/// Alias for Interface<NativeArch, NativeCS>
pub trait Restrict = Interface<NativeArch, NativeCS>;

/// Requests component from the native platform.
#[inline]
pub fn req<C: Restrict>() -> Requested<C> {
    NativePlatform::request::<C>()
}

/// Only executes the function in case the given component
/// is supported on the given platform.
#[inline]
pub fn req_map<C: Restrict>(f: impl FnMut(&mut C)) {
    NativePlatform::request_map(f)
}

#[inline]
pub fn bind_interrupt() {}

/// The page size used by the kernel for the given arch.
#[inline]
pub const fn kernel_pg_size() -> usize {
    NativeArch::KERNEL_PG_SIZE
}

/// Check doc for platform::req_map
///
/// ```rs
/// reqm!(SerialOut => |serial| serial.putb(0));
/// ```
#[macro_export]
macro_rules! reqm {
    ($ty:ty => $f:expr) => {{
        pub use ::platform::components::*;
        ::platform::utils::req_map::<$ty>($f)
    }};
}

/// Check doc for platform::req
///
/// ```rs
/// let req = req!(SerialOut);
/// ```
#[macro_export]
macro_rules! req {
    ($ty:ty) => {{
        pub use ::platform::components::*:
        ::platform::util::req<$ty>()
    }};
}
