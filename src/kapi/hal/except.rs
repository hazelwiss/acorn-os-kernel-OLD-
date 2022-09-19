use crate::{arch::get_arch_interfaces, once};

pub enum ExceptType {}

pub struct IDesc {
    pub init: fn(),
    /// global disable exceptions.
    pub gdexc: fn(),
    /// global enable exceptions
    pub geexc: fn(),
    /// disable exception
    pub dexc: fn(ExceptType),
    /// enable exception
    pub eexc: fn(ExceptType),
}

static IEXCEPT: IDesc = get_arch_interfaces().except;

pub fn init() {
    once! {
        (IEXCEPT.init)();
    }
}
