use crate::{hal::except::ExceptType, kapi::arch, once};

pub fn init() {
    once! {
        arch::cpu::amd64::except::init();
    }
}

pub fn gdexc() {
    todo!()
}

pub fn geexc() {
    todo!()
}

pub fn dexc(ty: ExceptType) {
    todo!()
}

pub fn eexc(ty: ExceptType) {
    todo!()
}
