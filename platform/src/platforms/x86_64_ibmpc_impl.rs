mod isr;

use crate::{
    platforms::{ibmpc as CS, x86_64 as Arch, ExceptType, IRQType, PlatformImpl},
    Platform,
};
#[allow(unused)]
use arch::{cpu::x86_64 as cpu, fam::x86 as fam};

const IRQ_START_VEC: u8 = 0x20;
const EXCEPT_START_VEC: u8 = 0x00;

impl Platform<Arch, CS> {
    fn init_irqs() {
        use fam::pic;
        pic::remap(IRQ_START_VEC, IRQ_START_VEC + 8);
        pic::disable_all();
        // Masks only keyboard interrupts.
        pic::pic1::mask(0xFF ^ 2);
        isr::init_irqs(IRQ_START_VEC);
    }

    fn init_excepts() {
        isr::init_excepts(EXCEPT_START_VEC);
    }
}

impl PlatformImpl for Platform<Arch, CS> {
    fn initialize() {
        Self::init_irqs();
        Self::init_excepts();
        cpu::idt::install();
        fam::sti();
    }

    fn reg_irq_fn(ty: IRQType) {
        unsafe {
            match ty {
                IRQType::Kbd(f) => *isr::KBD = f,
                IRQType::Arch(_f) => {}
            }
        }
    }

    fn reg_exc_fn(ty: ExceptType) {
        unsafe {
            match ty {
                ExceptType::PgFault(f) => *isr::PGFLT = f,
                ExceptType::Arch(_f) => {}
            }
        }
    }
}
