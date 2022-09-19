use crate::{arch::get_arch_interfaces, once};

pub struct IDesc {
    pub init: fn(),
    pub putb: fn(u8),
    pub getb: fn() -> Option<u8>,
    pub empty_buf: fn(),
    pub buf_size: fn() -> usize,
}

static KBD: IDesc = get_arch_interfaces().keyboard;

pub fn init() {
    once!()
}

pub fn putb(b: u8) {
    (KBD.putb)(b)
}

pub fn getb() -> Option<u8> {
    (KBD.getb)()
}

pub fn empty_buf() {
    (KBD.empty_buf)()
}

pub fn buf_size() -> usize {
    (KBD.buf_size)()
}
