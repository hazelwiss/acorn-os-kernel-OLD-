ENTRY(stage1)

SECTIONS {
    __kernel_load_adr = 0x01000000;
    __kernel_stack_top = 0x00EFFFF0;
    __heap_size = 16M; 

    . = 0x500;
    __disk_load_buffer = .;
    . += 512;
    . = ALIGN(0x1000);
    __pml4t_adr = .;
    . += 0x1000;
    __pd = .;

    .bootstrap 0x7C00 : AT (0) {
        *(.boot.stage1)
        __bootstrap_load_adr = .;
        *(.boot .boot.*)
        . = ALIGN(512);
    }
    __bootstrap_size = SIZEOF(.bootstrap);

    .kernel __kernel_load_adr : AT(__bootstrap_size) {
        __kernel_load_beg = .;
        *(.setup)
        *(.text .text.*)
        *(.rodata .rodata.*)
        *(.data .data.*)
        *(.eh_frame)
        *(.eh_frame_hdr)
        *(.bss .bss.*)
        . = ALIGN(4K);
        __heap_start = .;
        . += __heap_size;
        __heap_end = .;
        . = ALIGN(512);
        __kernel_load_size = . - __kernel_load_beg;
    }
} 