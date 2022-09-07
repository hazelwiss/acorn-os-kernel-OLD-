use crate::{hal::irq::InterruptType, kapi::arch, once};

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

pub fn dirq(ty: InterruptType) {
    todo!()
}

pub fn eirq(ty: InterruptType) {
    todo!()
}

pub fn irq_kbd() {}
