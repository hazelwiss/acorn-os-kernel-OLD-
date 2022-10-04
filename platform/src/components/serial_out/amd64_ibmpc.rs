use super::{Requested, SerialOut};
use hal::serial::ISerialOut;
use hw::serial::com;

pub fn init() {
    com::init();
}

pub fn request() -> Requested<SerialOut> {
    Requested::Requested(SerialOut(ISerialOut {
        putb: putb,
        putv: putv,
    }))
}

fn putb(b: u8) {
    com::putb(com::Port::COM1, b)
}

fn putv(v: &[u8]) {
    for &b in v {
        com::putb(com::Port::COM1, b);
    }
}
