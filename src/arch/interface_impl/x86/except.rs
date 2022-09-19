use crate::{arch, hal::except::ExceptType, once};

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

pub fn dexc(_ty: ExceptType) {
    todo!()
}

pub fn eexc(_ty: ExceptType) {
    todo!()
}
