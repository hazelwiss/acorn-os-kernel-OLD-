use core::arch::global_asm;

global_asm!(include_str!("stage1.s"));
global_asm!(include_str!("stage2.s"));
global_asm!(include_str!("stage3.s"));
global_asm!(include_str!("stage4.s"));
global_asm!(include_str!("vga.s"));
global_asm!(include_str!("gdt_defs.s"));
