//! COM serial hardware.

use arch::fam::x86::io::{in8, out8};
use spin::Mutex;

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum Port {
    COM1 = 0x3F8,
    COM2 = 0x2F8,
    COM3 = 0x3E8,
    COM4 = 0x2E8,
    COM5 = 0x5F8,
    COM6 = 0x4F8,
    COM7 = 0x5E8,
    COM8 = 0x4E8,
}

/// Initializes the COM serial ports.
pub fn init() {
    init_port(Port::COM1);
}

/// Puts a byte onto a COM serial port.
pub fn putb(port: Port, b: u8) {
    lock(port, || out8(port as u16, b));
}

static LOCKS: [Mutex<()>; 8] = [
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
];

fn get_lock(port: Port) -> &'static Mutex<()> {
    &LOCKS[match port {
        Port::COM1 => 0,
        Port::COM2 => 1,
        Port::COM3 => 2,
        Port::COM4 => 3,
        Port::COM5 => 4,
        Port::COM6 => 5,
        Port::COM7 => 6,
        Port::COM8 => 7,
    }]
}

fn lock(port: Port, mut f: impl FnMut()) {
    let _lock = get_lock(port).lock();
    f();
}

fn init_port(port: Port) {
    lock(port, || {
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
            panic!(
                "Unable to properly configure COM port {:?} for serial.",
                port
            )
        }
        out8(port_adr + 4, 0x0F);
    });
}
