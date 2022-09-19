use crate::{arch::get_arch_interfaces, once};

pub struct IDesc {
    pub init: fn(),
}

const FB: IDesc = get_arch_interfaces().fb;

pub fn init() {
    once!(
        (FB.init)();
    )
}
