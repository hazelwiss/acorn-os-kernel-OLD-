use core::arch::global_asm;

global_asm!(include_str!("amd64/stage1.s"));
global_asm!(include_str!("amd64/stage2.s"));
global_asm!(include_str!("amd64/stage3.s"));
global_asm!(include_str!("amd64/stage4.s"));
global_asm!(include_str!("amd64/vga.s"));
global_asm!(include_str!("amd64/gdt_defs.s"));
