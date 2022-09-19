use super::Driver;
use crate::kapi::hal::irq;
use spin::Mutex;

pub struct State {}

impl Driver for State {
    fn init(&self) {}

    fn name(&self) -> &'static str {
        "IRQ driver"
    }
}

impl State {
    pub fn enable(&self) {
        irq::geirq()
    }

    pub fn disable(&self) {
        irq::gdirq()
    }

    pub fn wait(&self) {
        irq::wait()
    }
}

static IRQ: Mutex<State> = Mutex::new(State {});

pub fn enable() {
    IRQ.lock().enable()
}

pub fn disable() {
    IRQ.lock().disable()
}

pub fn wait() {
    IRQ.lock().wait()
}
