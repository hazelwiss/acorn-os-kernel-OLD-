use super::Driver;
use crate::hal;
use kutil::once;
use spin::Mutex;

struct Desc {}

impl Driver for Desc {
    fn init(&self) {
        todo!()
    }

    fn name(&self) -> &'static str {
        "Serial out driver."
    }
}

impl Desc {
    pub fn putc(&self, c: char) {
        match c {
            '\n' | '\0' => {
                hal::serial::putb('\n' as u8);
                hal::serial::putb('\r' as u8);
            }
            c => hal::serial::putb(c as u8),
        };
    }

    pub fn puts(&self, s: &str) {
        for c in s.chars() {
            self.putc(c);
        }
    }
}

const SERIAL_OUT: Mutex<Desc> = Mutex::new(Desc {});

pub fn init() {
    once!();
}

pub fn putc(c: char) {
    SERIAL_OUT.lock().putc(c);
}

pub fn puts(s: &str) {
    SERIAL_OUT.lock().puts(s);
}
