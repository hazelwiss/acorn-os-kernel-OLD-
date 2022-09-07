//! Framebuffer interface
use crate::{arch::get_arch_interfaces, once};
use spin::Mutex;

pub struct IDesc {
    pub init: fn(),
}

const FB: Mutex<&IDesc> = Mutex::new(&get_arch_interfaces().fb);

pub fn init() {
    once!(
        (FB.lock().init)();
    )
}
