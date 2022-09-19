use super::Driver;
use crate::kapi::{hal, once};
use spin::Mutex;

struct State {}

impl Driver for State {
    fn init(&self) {
        once!(
            hal::serial::init();
        );
    }

    fn name(&self) -> &'static str {
        "Serial out driver."
    }
}

impl State {
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

static SERIAL_OUT: Mutex<State> = Mutex::new(State {});

pub fn init() {
    SERIAL_OUT.lock().init()
}

pub fn putc(c: char) {
    SERIAL_OUT.lock().putc(c);
}

pub fn puts(s: &str) {
    SERIAL_OUT.lock().puts(s);
}
