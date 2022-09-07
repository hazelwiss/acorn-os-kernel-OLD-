use spin::Mutex;

use crate::{arch::get_arch_interfaces, once};

pub enum InterruptType {
    Keyboard,
}

pub struct IDesc {
    pub init: fn(),
    /// global disable irq
    pub gdirq: fn(),
    /// global enable irq
    pub geirq: fn(),
    /// disable interrupt
    pub dirq: fn(InterruptType),
    /// enable interrupt
    pub eirq: fn(InterruptType),
    // different interrupts
    pub irq_kbd: fn(),
}

static IIRQ: Mutex<IDesc> = Mutex::new(get_arch_interfaces().irq);

pub fn init() {
    once!(
        (IIRQ.lock().init)();
    )
}

/// global disable irq
pub fn gdirq() {
    (IIRQ.lock().gdirq)();
}

/// global enable irq
pub fn geirq() {
    (IIRQ.lock().geirq)();
}
