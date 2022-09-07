ENTRY(stage1)
OUTPUT_FORMAT(elf64-x86-64)
OUTPUT_ARCH(i386:x86-64)

SECTIONS {
    __kernel_load_adr = 0x01000000;
    __kernel_virt_adr = 0xFFFFFFFF80000000;
    __bootstrap_bss_base_adr = 0x00100000;

    . = 0x500;
    __disk_load_buffer = .;
    . += 512;

    .boot.start 0x7C00 : AT(0) {
        KEEP(*(.boot.start))
    }

    __bootstrap_load_adr = .;
    .boot.text __bootstrap_load_adr : AT(SIZEOF(.boot.start)) {
        *(.boot.text .boot.text.*)
        . = ALIGN(512);
    }
    __bootstrap_load_size = SIZEOF(.boot.text);

    .boot.bss __bootstrap_bss_base_adr (NOLOAD) : AT(SIZEOF(.boot.text) + SIZEOF(boot.start)) {
        __boot_bss_start = .;
        *(.boot.bss .boot.bss.*)
        . = ALIGN(4K);
        __p4 = .;
        . += 4K;
        __p3 = .;
        . += 4K;
        __p2 = .;
        . += 4K * 2;
        __p1 = .;
        . += 4K * 512 * 2;
        __boot_bss_end = .;
    }

    . = __kernel_load_adr + __kernel_virt_adr;
    __kernel_stack_top = .;

    .setup ALIGN(4K) : AT(ADDR(.setup) - __kernel_virt_adr) {
        KEEP(*(.setup))
    }

    .text ALIGN(4K) : AT(ADDR(.text) - __kernel_virt_adr) {
        *(.text .text.*)
    }

    .rodata ALIGN(4K) : AT(ADDR(.rodata) - __kernel_virt_adr) {
        *(.rodata .rodata.*)
    }

    .data ALIGN(4K) : AT(ADDR(.data) - __kernel_virt_adr) {
        *(.data .data.*)
    }

    .misc ALIGN(4K) : AT(ADDR(.misc) - __kernel_virt_adr) {
        *(.eh_frame)
        *(.eh_frame_hdr)
    }

    __bss_start = .;

    .bss ALIGN(4K) : AT(ADDR(.bss) - __kernel_virt_adr) {
		*(COMMON)
        *(.bss .bss.*)
    }

    .bss.noload ALIGN(4K) (NOLOAD) : AT(ADDR(.bss.noload) - __kernel_virt_adr)
	{
		*(.bss.noload)
        . = ALIGN(4K);
        __wma_start = .;
        . += 48M;
        __wma_end = .;
	}

    __bss_end = .;

    . = ALIGN(4K);
    __kernel_size = . - __kernel_virt_adr;
} 

