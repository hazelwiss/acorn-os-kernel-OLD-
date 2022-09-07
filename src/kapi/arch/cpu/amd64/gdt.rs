#[repr(C, packed)]
struct Entry {
    limit_low: u16,
    base_low: u16,
    base_low_ext: u8,
    access_byte: u8,
    limit_and_flags: u8,
    base_hi: u8,
}

impl Entry {
    const fn new_null() -> Self {
        unsafe { core::mem::MaybeUninit::zeroed().assume_init() }
    }

    const fn set_limit(mut self, limit: u32) -> Self {
        assert!(limit <= 0xFFFFF);
        self.limit_low = limit as u16;
        self.limit_and_flags &= 0x0F;
        self.limit_and_flags |= (limit >> 12) as u8 & 0xF0;
        self
    }

    const fn set_base(mut self, base: u64) -> Self {
        self.base_low = base as u16;
        self.base_low_ext = (base >> 16) as u8;
        self.base_hi = (base >> 24) as u8;
        self
    }

    const fn set_access_byte(mut self, access_byte: u8) -> Self {
        self.access_byte = access_byte;
        self
    }

    const fn set_flags(mut self, limit: u8) -> Self {
        assert!(limit <= 0x0F);
        self.limit_and_flags &= 0xF0;
        self.limit_and_flags |= limit & 0x0F;
        self
    }
}

const ENTRY_SIZE: u16 = core::mem::size_of::<Entry>() as u16;
const _ASSERT_SIZE: () = assert!(ENTRY_SIZE == 8);

static GDT: [Entry; 5] = [
    // null entry
    Entry::new_null(),
    // kernel code
    Entry::new_null()
        .set_limit(0x0)
        .set_access_byte(0x9A)
        .set_flags(0xA)
        .set_base(0),
    // kernel data
    Entry::new_null()
        .set_limit(0x0)
        .set_access_byte(0x92)
        .set_flags(0xC)
        .set_base(0),
    // userspace code
    Entry::new_null()
        .set_limit(0x0)
        .set_access_byte(0xFA)
        .set_flags(0xA)
        .set_base(0),
    // userspace data
    Entry::new_null()
        .set_limit(0x0)
        .set_access_byte(0xF2)
        .set_flags(0xC)
        .set_base(0),
];

pub const KERNEL_CODE_SELECTOR: u16 = ENTRY_SIZE * 1;
pub const KERNEL_DATA_SELECTOR: u16 = ENTRY_SIZE * 2;
pub const USRSPC_CODE_SELECTOR: u16 = ENTRY_SIZE * 3;
pub const USRSPC_DATA_SELECTOR: u16 = ENTRY_SIZE * 4;

pub fn apply_table() {}
