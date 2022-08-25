pub mod chipset;
pub mod cpu;
pub mod family;

use crate::hal;

pub struct ArchInterfaces {
    pub iconsole: hal::console::IDesc,
    pub ifb: hal::fb::IDesc,
    pub iserial: hal::serial::IDesc,
}

pub const fn get_arch_interfaces() -> ArchInterfaces {
    get_arch_interfaces_x86()
}

const fn get_arch_interfaces_x86() -> ArchInterfaces {
    ArchInterfaces {
        iconsole: hal::console::IDesc {
            init: chipset::x86::fb::init,
            putc: chipset::x86::fb::putc,
            puts: chipset::x86::fb::puts,
            clear: chipset::x86::fb::clear,
            get_w: chipset::x86::fb::get_w,
            get_h: chipset::x86::fb::get_h,
            new_line: chipset::x86::fb::new_line,
        },
        ifb: hal::fb::IDesc {
            init: chipset::x86::fb::init,
        },
        iserial: hal::serial::IDesc {
            init: chipset::x86::com::init,
            putb: chipset::x86::com::putb,
        },
    }
}
