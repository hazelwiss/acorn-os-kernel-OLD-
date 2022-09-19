use crate::{arch::get_arch_interfaces, once};

pub struct IDesc {
    pub init: fn(),
    pub putb: fn(u8),
}

const ISERIAL: IDesc = get_arch_interfaces().serial;

pub fn init() {
    once!(
        (ISERIAL.init)();
    )
}

pub fn putb(b: u8) {
    (ISERIAL.putb)(b);
}
