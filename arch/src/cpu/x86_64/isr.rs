//! Interrupt service routine.
//! Contains macros.

use core::fmt::Display;

/// A stack frame for retaining register values
/// throughout interrupts.
#[repr(C, packed)]
pub struct StackFrame {
    rbp: u64,
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rdi: i64,
    rsi: u64,
    rdx: u64,
    rcx: u64,
    rbx: u64,
    rax: u64,
    id: u64,
    rip: u64,
    cs: u64,
    rf: u64,
    rsp: u64,
    ss: u64,
}

impl Display for StackFrame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self {
            ss,
            rsp,
            rf,
            cs,
            rip,
            id,
            rax,
            rbx,
            rcx,
            rdx,
            rsi,
            rdi,
            r8,
            r9,
            r10,
            r11,
            r12,
            r13,
            r14,
            r15,
            rbp,
        } = *self;
        f.write_fmt(format_args!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
            format_args!("ss:  {ss:016X}"),
            format_args!("rsp: {rsp:016X}"),
            format_args!("rf:  {rf:016X}"),
            format_args!("cs:  {cs:016X}"),
            format_args!("rip: {rip:016X}"),
            format_args!("id:  {id:016X}"),
            format_args!("rax: {rax:016X}"),
            format_args!("rbx: {rbx:016X}"),
            format_args!("rcx: {rcx:016X}"),
            format_args!("rdx: {rdx:016X}"),
            format_args!("rsi: {rsi:016X}"),
            format_args!("rdi: {rdi:016X}"),
            format_args!("r8:  {r8:016X}"),
            format_args!("r9:  {r9:016X}"),
            format_args!("r10: {r10:016X}"),
            format_args!("r11: {r11:016X}"),
            format_args!("r12: {r12:016X}"),
            format_args!("r13: {r13:016X}"),
            format_args!("r14: {r14:016X}"),
            format_args!("r15: {r15:016X}"),
            format_args!("rbp: {rbp:016X}"),
        ))
    }
}
