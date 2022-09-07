use super::pic;
use crate::{arch::family::x86::port::in8, once};
use spin::Mutex;

struct Keyboard {
    read: Option<u8>,
}

impl Keyboard {
    fn getb(&mut self) -> Option<u8> {
        self.read.take()
    }

    fn getc(&mut self) -> Option<char> {
        if let Some(b) = self.getb() {
            match b {
                _ => Some('A'),
            }
        } else {
            None
        }
    }

    fn putb(&self, b: u8) {
        todo!()
    }

    fn add_to_queue(&mut self, b: u8) {
        self.read = Some(b)
    }

    fn interrupt_handler(&mut self) {
        let read = in8(0x60);
    }
}

static KBD: Mutex<Keyboard> = Mutex::new(Keyboard { read: None });

pub fn init() {
    once!()
}

pub fn getb() -> Option<u8> {
    KBD.lock().getb()
}

pub fn getc() -> Option<char> {
    KBD.lock().getc()
}

pub fn putb(b: u8) {
    KBD.lock().putb(b)
}

#[inline]
pub fn interrupt_handler() {
    KBD.lock().interrupt_handler();
}
