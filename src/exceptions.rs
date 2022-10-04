//! Exception handlers.

pub fn pg_fault(_adr: usize) {}

pub fn arch_specific(id: usize) {
    panic!("arch specific exception {id}");
}
