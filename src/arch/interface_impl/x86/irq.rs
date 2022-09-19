use crate::{arch, hal::irq::InterruptType, once};

pub fn init() {
    once! {
        arch::cpu::amd64::irq::init();
    }
}

pub fn gdirq() {
    arch::cpu::amd64::irq::disable();
}

pub fn geirq() {
    arch::cpu::amd64::irq::enable();
}

pub fn dirq(_ty: InterruptType) {
    todo!()
}

pub fn eirq(_ty: InterruptType) {
    todo!()
}

pub fn wait() {
    arch::cpu::amd64::irq::wait()
}
