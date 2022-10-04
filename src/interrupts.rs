//! Interrupt handlers.

pub fn kbd_interrupt(_id: usize) {}

pub fn arch_specific(id: usize) {
    panic!("arch specific interrupt {id}");
}
