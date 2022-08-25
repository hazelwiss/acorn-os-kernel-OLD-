use crate::arch::get_arch_interfaces;
use spin::Mutex;

pub struct IDesc {
    pub init: fn(),
    pub putb: fn(u8),
}

const ISERIAL: Mutex<&IDesc> = Mutex::new(&get_arch_interfaces().iserial);

pub fn putb(b: u8) {
    (ISERIAL.lock().putb)(b);
}
