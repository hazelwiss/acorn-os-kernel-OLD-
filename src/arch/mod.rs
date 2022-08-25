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
            init: family::x86::fb::init,
            putc: family::x86::fb::putc,
            puts: family::x86::fb::puts,
            clear: family::x86::fb::clear,
            get_w: family::x86::fb::get_w,
            get_h: family::x86::fb::get_h,
        },
        ifb: hal::fb::IDesc {
            init: family::x86::fb::init,
        },
        iserial: hal::serial::IDesc {
            init: family::x86::com::init,
            putb: family::x86::com::putb,
        },
    }
}
