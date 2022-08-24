pub mod drivers {
    use core::sync::atomic::{AtomicBool, Ordering};

    use crate::drivers;

    pub const SERIAL: &drivers::Desc = &drivers::x64::com::SERIAL;
    pub const SHELL: &drivers::Desc = &drivers::x64::console::CONSOLE;

    pub const INIT_ON_BOOT: &[&'static drivers::Desc] = &[&SERIAL, &SHELL];
    pub static INITIALIZED: AtomicBool = AtomicBool::new(false);

    pub fn init() {
        for init in INIT_ON_BOOT {
            (init.entry)();
        }
        INITIALIZED.store(true, Ordering::Relaxed);
    }
}

pub mod cpu {}

pub mod family {}
