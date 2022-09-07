mod except;
mod irq;

use crate::{
    arch::chipset,
    hal::{self, ArchInterfaces},
};

pub const fn get_arch_interfaces() -> ArchInterfaces {
    ArchInterfaces {
        console: hal::console::IDesc {
            init: chipset::x86::fb::init,
            putc: chipset::x86::fb::putc,
            puts: chipset::x86::fb::puts,
            clear: chipset::x86::fb::clear,
            get_w: chipset::x86::fb::get_w,
            get_h: chipset::x86::fb::get_h,
            new_line: chipset::x86::fb::new_line,
        },
        fb: hal::fb::IDesc {
            init: chipset::x86::fb::init,
        },
        serial: hal::serial::IDesc {
            init: chipset::x86::com::init,
            putb: chipset::x86::com::putb,
        },
        irq: hal::irq::IDesc {
            init: irq::init,
            gdirq: irq::gdirq,
            geirq: irq::geirq,
            dirq: irq::dirq,
            eirq: irq::eirq,
            irq_kbd: irq::irq_kbd,
        },
        except: hal::except::IDesc {
            init: except::init,
            gdexc: except::gdexc,
            geexc: except::geexc,
            dexc: except::dexc,
            eexc: except::eexc,
        },
        keyboard: hal::keyboard::IDesc {
            init: chipset::x86::kbd::init,
            putb: chipset::x86::kbd::putb,
            getb: chipset::x86::kbd::getb,
            getc: chipset::x86::kbd::getc,
        },
    }
}
