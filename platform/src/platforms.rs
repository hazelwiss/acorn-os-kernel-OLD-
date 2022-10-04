#[cfg(target_arch = "x86_64")]
mod x86_64_ibmpc_impl;

proc_macros::targets!();

pub(crate) fn init<P: PlatformRestrict>() {
    P::initialize();
}
