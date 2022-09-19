use crate::hal::irq;
use spin::Mutex;

struct State {}

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
