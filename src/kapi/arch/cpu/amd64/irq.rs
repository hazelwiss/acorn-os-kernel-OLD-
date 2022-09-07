use crate::{
    arch::{chipset::x86::pic, cpu::amd64::idt},
    kapi::arch::chipset::x86::kbd,
    once,
};
use core::arch::asm;

use super::isr::StackFrame;

pub fn is_irq_enabled() -> bool {
    todo!()
}

pub fn enable() {
    unsafe { asm!("sti") }
}

pub fn disable() {
    unsafe { asm!("cli") }
}

pub fn wait() {
    unsafe { asm!("hlt") }
}

extern "C" {
    fn __irq_handler0();
    fn __irq_handler1();
    fn __irq_handler2();
    fn __irq_handler3();
    fn __irq_handler4();
    fn __irq_handler5();
    fn __irq_handler6();
    fn __irq_handler7();
    fn __irq_handler8();
    fn __irq_handler9();
    fn __irq_handler10();
    fn __irq_handler11();
    fn __irq_handler12();
    fn __irq_handler13();
    fn __irq_handler14();
    fn __irq_handler15();

}

// Id 0
#[no_mangle]
unsafe extern "C" fn piti(_stack_frame: *const StackFrame) -> *const StackFrame {
    pic::pic1::end_of_interrupt();
    panic!("piti interrupt");
}

// Id 1
#[no_mangle]
unsafe extern "C" fn kbd(stack_frame: *const StackFrame) -> *const StackFrame {
    kbd::interrupt_handler();
    pic::pic1::end_of_interrupt();
    stack_frame
}

// Id 2
#[no_mangle]
unsafe extern "C" fn cascade(_stack_frame: *const StackFrame) -> *const StackFrame {
    pic::pic1::end_of_interrupt();
    panic!("cascade interrupt")
}

// Id 3
#[no_mangle]
unsafe extern "C" fn com2(_stack_frame: *const StackFrame) -> *const StackFrame {
    pic::pic1::end_of_interrupt();
    panic!("com2 interrupt")
}

// Id 4
#[no_mangle]
unsafe extern "C" fn com1(_stack_frame: *const StackFrame) -> *const StackFrame {
    pic::pic1::end_of_interrupt();
    panic!("com1 interrupt")
}

// Id 5
#[no_mangle]
unsafe extern "C" fn lpt2(_stack_frame: *const StackFrame) -> *const StackFrame {
    pic::pic1::end_of_interrupt();
    panic!("lpt2 interrupt")
}

// Id 6
#[no_mangle]
unsafe extern "C" fn floppy(_stack_frame: *const StackFrame) -> *const StackFrame {
    pic::pic1::end_of_interrupt();
    panic!("floppy interrupt")
}

// Id 7
#[no_mangle]
unsafe extern "C" fn lpt1(_stack_frame: *const StackFrame) -> *const StackFrame {
    pic::pic1::end_of_interrupt();
    panic!("lpt1 interrupt")
}

// Id 8
#[no_mangle]
unsafe extern "C" fn rtc(_stack_frame: *const StackFrame) -> *const StackFrame {
    pic::end_of_interrupt();
    panic!("rtc interrupt")
}

// Id 9
#[no_mangle]
unsafe extern "C" fn free0(_stack_frame: *const StackFrame) -> *const StackFrame {
    pic::end_of_interrupt();
    panic!("free0 interrupt")
}

// Id 10
#[no_mangle]
unsafe extern "C" fn free1(_stack_frame: *const StackFrame) -> *const StackFrame {
    pic::end_of_interrupt();
    panic!("free1 interrupt")
}

// Id 11
#[no_mangle]
unsafe extern "C" fn free2(_stack_frame: *const StackFrame) -> *const StackFrame {
    pic::end_of_interrupt();
    panic!("free2 interrupt")
}

// Id 12
#[no_mangle]
unsafe extern "C" fn mouse(_stack_frame: *const StackFrame) -> *const StackFrame {
    pic::end_of_interrupt();
    panic!("mouse interrupt")
}

// Id 13
#[no_mangle]
unsafe extern "C" fn cp(_stack_frame: *const StackFrame) -> *const StackFrame {
    pic::end_of_interrupt();
    panic!("cp interrupt")
}

// Id 14
#[no_mangle]
unsafe extern "C" fn pata(_stack_frame: *const StackFrame) -> *const StackFrame {
    pic::end_of_interrupt();
    panic!("pata interrupt")
}

// Id 15
#[no_mangle]
unsafe extern "C" fn sata(_stack_frame: *const StackFrame) -> *const StackFrame {
    pic::end_of_interrupt();
    panic!("sata interrupt")
}

pub fn init() {
    once! {
        pic::init();
        pic::pic1::disable_all();
        pic::pic2::disable_all();
        pic::pic1::mask(0xFF ^ 2);
        idt::init();
        idt::set_descriptor(0x20, __irq_handler0);
        idt::set_descriptor(0x21, __irq_handler1);
        idt::set_descriptor(0x22, __irq_handler2);
        idt::set_descriptor(0x23, __irq_handler3);
        idt::set_descriptor(0x24, __irq_handler4);
        idt::set_descriptor(0x25, __irq_handler5);
        idt::set_descriptor(0x26, __irq_handler6);
        idt::set_descriptor(0x27, __irq_handler7);
        idt::set_descriptor(0x28, __irq_handler8);
        idt::set_descriptor(0x29, __irq_handler9);
        idt::set_descriptor(0x2A, __irq_handler10);
        idt::set_descriptor(0x2B, __irq_handler11);
        idt::set_descriptor(0x2C, __irq_handler12);
        idt::set_descriptor(0x2D, __irq_handler13);
        idt::set_descriptor(0x2E, __irq_handler14);
        idt::set_descriptor(0x2F, __irq_handler15);
        idt::install();
    }
}
