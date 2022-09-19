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
    /// waits for interrupt
    pub wait: fn(),
}

static IIRQ: IDesc = get_arch_interfaces().irq;

pub fn init() {
    once!(
        (IIRQ.init)();
    )
}

/// global disable irq
pub fn gdirq() {
    (IIRQ.gdirq)();
}

/// global enable irq
pub fn geirq() {
    (IIRQ.geirq)();
}

/// halts program execution
pub fn wait() {
    (IIRQ.wait)()
}
