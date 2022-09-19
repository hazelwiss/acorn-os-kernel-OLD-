use crate::{arch::family::x86::port::in8, once};
use spin::Mutex;

struct Keyboard {
    read: Option<u8>,
}

impl Keyboard {
    fn getb(&mut self) -> Option<u8> {
        self.read.take()
    }

    fn putb(&self, _b: u8) {
        todo!()
    }

    fn add_to_queue(&mut self, b: u8) {
        self.read = Some(b)
    }

    fn empty_buf(&mut self) {
        self.read = None;
    }

    fn interrupt_handler(&mut self) {
        let read = in8(0x60);
        self.add_to_queue(read);
    }
}

static KBD: Mutex<Keyboard> = Mutex::new(Keyboard { read: None });

pub fn init() {
    once!()
}

pub fn getb() -> Option<u8> {
    KBD.lock().getb()
}

pub fn putb(b: u8) {
    KBD.lock().putb(b)
}

pub fn buf_size() -> usize {
    1
}

pub fn empty_buf() {
    KBD.lock().empty_buf();
}

#[inline]
pub fn interrupt_handler() {
    KBD.lock().interrupt_handler();
}
