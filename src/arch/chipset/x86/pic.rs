use crate::{
    arch::family::x86::port::{in8_delay, out8_delay},
    once,
};
use spin::Mutex;

const PIC1_BASE: u16 = 0x20;
const PIC2_BASE: u16 = 0xA0;

const COMMAND_OFFSET: u16 = 0;
const DATA_OFFSET: u16 = 1;

pub mod cmd {
    pub const EOI: u8 = 0x20;
    pub const ICW1_INIT: u8 = 0x10;
    pub const ICW4_8086: u8 = 0x01;
}

struct Pic {
    base: u16,
    cascade_ident: u8,
}

impl Pic {
    fn init(&self) {
        once!()
    }

    fn remap(&self, start_vec: u8) {
        let mask = self.data_in();
        self.command(cmd::ICW1_INIT);
        self.data_out(start_vec);
        self.data_out(self.cascade_ident);
        self.data_out(cmd::ICW4_8086);
        self.data_out(mask);
    }

    #[inline]
    fn command(&self, cmd: u8) {
        out8_delay(self.base + COMMAND_OFFSET, cmd);
    }

    #[inline]
    fn data_out(&self, out: u8) {
        out8_delay(self.base + DATA_OFFSET, out);
    }

    #[inline]
    fn data_in(&self) -> u8 {
        in8_delay(self.base + DATA_OFFSET)
    }
}

static PIC1: Mutex<Pic> = Mutex::new(Pic {
    base: PIC1_BASE,
    cascade_ident: 4,
});
static PIC2: Mutex<Pic> = Mutex::new(Pic {
    base: PIC2_BASE,
    cascade_ident: 2,
});

pub fn init() {
    once!(
        PIC1.lock().init();
        PIC2.lock().init();
        // given interrupt 0 - 31 is reserved by
        // intel for exceptions, remap them.
        remap(0x20, 0x28);
    );
}

pub fn remap(pic1_vec: u8, pic2_vec: u8) {
    pic1::remap(pic1_vec);
    pic2::remap(pic2_vec);
}

pub fn cmd(cmd: u8) {
    pic1::cmd(cmd);
    pic2::cmd(cmd);
}

pub fn end_of_interrupt() {
    cmd(cmd::EOI)
}

pub mod pic1 {
    use super::*;

    pub fn remap(start_vec: u8) {
        PIC1.lock().remap(start_vec);
    }

    pub fn mask(mask: u8) {
        PIC1.lock().data_out(mask);
    }

    pub fn cmd(cmd: u8) {
        PIC1.lock().command(cmd);
    }

    pub fn end_of_interrupt() {
        cmd(cmd::EOI)
    }

    pub fn enable_all() {
        mask(0);
    }

    pub fn disable_all() {
        mask(0xFF);
    }
}

pub mod pic2 {
    use super::*;

    pub fn remap(start_vec: u8) {
        PIC2.lock().remap(start_vec);
    }

    pub fn mask(mask: u8) {
        PIC2.lock().data_out(mask);
    }

    pub fn cmd(cmd: u8) {
        PIC2.lock().command(cmd);
    }

    pub fn end_of_interrupt() {
        cmd(cmd::EOI)
    }

    pub fn enable_all() {
        mask(0);
    }

    pub fn disable_all() {
        mask(0xFF);
    }
}
