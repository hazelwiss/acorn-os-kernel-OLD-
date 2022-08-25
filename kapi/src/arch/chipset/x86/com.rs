use spin::mutex::Mutex;

use crate::{
    arch::family::x86::port::{in8, out8},
    util::once,
};

pub fn init() {
    once!(
    init_port(COMPort::COM1, 115200);

    );
}

pub fn putb(b: u8) {
    putb_port(COMPort::COM1, b);
}

pub fn getb() -> u8 {
    getb_port(COMPort::COM1)
}

static WRITER_LOCK: [Mutex<usize>; 8] = [
    Mutex::new(0),
    Mutex::new(0),
    Mutex::new(0),
    Mutex::new(0),
    Mutex::new(0),
    Mutex::new(0),
    Mutex::new(0),
    Mutex::new(0),
];
static READER_LOCK: [Mutex<usize>; 8] = [
    Mutex::new(0),
    Mutex::new(0),
    Mutex::new(0),
    Mutex::new(0),
    Mutex::new(0),
    Mutex::new(0),
    Mutex::new(0),
    Mutex::new(0),
];

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(u16)]
pub enum COMPort {
    COM1 = 0x3F8,
    COM2 = 0x2F8,
    COM3 = 0x3E8,
    COM4 = 0x2E8,
    COM5 = 0x5F8,
    COM6 = 0x4F8,
    COM7 = 0x5E8,
    COM8 = 0x4E8,
}

impl COMPort {
    fn get_id(&self) -> usize {
        match self {
            COMPort::COM1 => 0,
            COMPort::COM2 => 1,
            COMPort::COM3 => 2,
            COMPort::COM4 => 3,
            COMPort::COM5 => 4,
            COMPort::COM6 => 5,
            COMPort::COM7 => 6,
            COMPort::COM8 => 7,
        }
    }
}

pub fn putb_port(port: COMPort, b: u8) {
    let _lock = WRITER_LOCK[port.get_id()].lock();
    out8(port as u16, b)
}

pub fn getb_port(port: COMPort) -> u8 {
    let _lock = READER_LOCK[port.get_id()].lock();
    in8(port as u16)
}

pub fn init_port(port: COMPort, _buad_rate: u32) {
    let port_adr = port as u16;
    out8(port_adr + 1, 0x00);
    out8(port_adr + 3, 0x80);
    out8(port_adr + 0, 0x03);
    out8(port_adr + 1, 0x00);
    out8(port_adr + 3, 0x03);
    out8(port_adr + 2, 0xC7);
    out8(port_adr + 4, 0x0B);
    out8(port_adr + 4, 0x1E);

    // Test connection.
    out8(port_adr, 0xEA);
    if in8(port_adr) != 0xEA {
        panic!("Unabel to properly configure COM port for serial.")
    }
    out8(port_adr + 4, 0x0F);
}
