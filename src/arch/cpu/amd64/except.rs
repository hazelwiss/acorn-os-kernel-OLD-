use crate::{
    arch::{chipset::x86::pic, cpu::amd64::idt},
    once,
};

extern "C" {
    fn __except_handler0();
    fn __except_handler1();
    fn __except_handler2();
    fn __except_handler3();
    fn __except_handler4();
    fn __except_handler5();
    fn __except_handler6();
    fn __except_handler7();
    fn __except_handler8();
    fn __except_handler9();
    fn __except_handler10();
    fn __except_handler11();
    fn __except_handler12();
    fn __except_handler13();
    fn __except_handler14();
    fn __except_handler15();
    fn __except_handler16();
    fn __except_handler17();
    fn __except_handler18();
    fn __except_handler19();
    fn __except_handler20();
    fn __except_handler21();
    fn __except_handler22();
    fn __except_handler23();
    fn __except_handler24();
    fn __except_handler25();
    fn __except_handler26();
    fn __except_handler27();
    fn __except_handler28();
    fn __except_handler29();
    fn __except_handler30();
    fn __except_handler31();
}

#[no_mangle]
unsafe extern "C" fn except() {
    logok!("EXCEPT")
}

pub fn init() {
    once! {
        pic::init();
        idt::init();
        idt::set_descriptor(0, __except_handler0);
        idt::set_descriptor(1, __except_handler1);
        idt::set_descriptor(2, __except_handler2);
        idt::set_descriptor(3, __except_handler3);
        idt::set_descriptor(4, __except_handler4);
        idt::set_descriptor(5, __except_handler5);
        idt::set_descriptor(6, __except_handler6);
        idt::set_descriptor(7, __except_handler7);
        idt::set_descriptor(8, __except_handler8);
        idt::set_descriptor(9, __except_handler9);
        idt::set_descriptor(10, __except_handler10);
        idt::set_descriptor(11, __except_handler11);
        idt::set_descriptor(12, __except_handler12);
        idt::set_descriptor(13, __except_handler13);
        idt::set_descriptor(14, __except_handler14);
        idt::set_descriptor(15, __except_handler15);
        idt::set_descriptor(16, __except_handler16);
        idt::set_descriptor(17, __except_handler17);
        idt::set_descriptor(18, __except_handler18);
        idt::set_descriptor(19, __except_handler19);
        idt::set_descriptor(20, __except_handler20);
        idt::set_descriptor(21, __except_handler21);
        idt::set_descriptor(22, __except_handler22);
        idt::set_descriptor(23, __except_handler23);
        idt::set_descriptor(24, __except_handler24);
        idt::set_descriptor(25, __except_handler25);
        idt::set_descriptor(26, __except_handler26);
        idt::set_descriptor(27, __except_handler27);
        idt::set_descriptor(28, __except_handler28);
        idt::set_descriptor(29, __except_handler29);
        idt::set_descriptor(30, __except_handler30);
        idt::set_descriptor(31, __except_handler31);
        idt::install();
    }
}
