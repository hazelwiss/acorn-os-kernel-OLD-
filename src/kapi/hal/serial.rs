use crate::{arch::get_arch_interfaces, once};
use spin::Mutex;

pub struct IDesc {
    pub init: fn(),
    pub putb: fn(u8),
}

const ISERIAL: Mutex<&IDesc> = Mutex::new(&get_arch_interfaces().serial);

pub fn init() {
    once!(
        (ISERIAL.lock().init)();
    )
}

pub fn putb(b: u8) {
    (ISERIAL.lock().putb)(b);
}
