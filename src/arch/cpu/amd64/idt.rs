use core::arch::asm;
use lazy_static::lazy_static;
use spin::Mutex;

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct IDTEntry {
    offset_1: u16,
    selector: u16,
    ist: u8,
    type_attributes: u8,
    offset_2: u16,
    offset_3: u32,
    _zero: u32,
}

impl IDTEntry {
    const fn new_null() -> Self {
        Self {
            offset_1: 0,
            selector: 0,
            ist: 0,
            type_attributes: 0,
            offset_2: 0,
            offset_3: 0,
            _zero: 0,
        }
    }
}

#[repr(C)]
#[repr(align(4096))]
struct IDT {
    entries: [IDTEntry; 256],
}

#[repr(C, packed)]
struct IDTR {
    size: u16,
    paged_offset: u64,
}

static IDT: Mutex<IDT> = Mutex::new(IDT {
    entries: [IDTEntry::new_null(); 256],
});

lazy_static! {
    static ref IDT_PTR: Mutex<IDTR> = {
        let idtr = IDTR {
            size: core::mem::size_of::<IDT>() as u16,
            paged_offset: &IDT.lock() as *const _ as u64,
        };
        Mutex::new(idtr)
    };
}

pub fn init() {
    apply()
}

pub fn add_descriptor(index: u8, interrupt_handler: fn()) {}

pub fn remove_descriptor(index: u8) {}

pub fn apply() {
    let idtr = IDT_PTR.lock();
    let idtr_ptr = (&*idtr) as *const IDTR as usize;
    unsafe {
        asm!(
            "lidt {}", 
            in(reg)(&idtr_ptr));
    }
}
