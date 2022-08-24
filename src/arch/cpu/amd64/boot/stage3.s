.section .boot, "awx"
.code32

.global 
stage3:
    cli # Clear interrupt flag.
    cld # Clear direction flag.

    call vga_clear
    mov esi, offset msg_hello
    call vga_println

    call check_cpuid
    call check_lm_support
    call check_enable_paging
    jmp enter_stage4

    mov esi, offset msg_halt
error32:
    call vga_println
halt32:
    hlt
    jmp halt32

check_cpuid:
    pushfd
    pushfd
    xor long ptr [esp], 0x00200000
    popfd
    pushfd
    pop eax
    xor eax, [esp]
    popfd
    and eax, 0x00200000
    jz cpuid_not_found
    ret
cpuid_not_found:
    mov esi, offset msg_no_cpuid_support
    jmp error32

check_lm_support:
    mov eax, 0x80000000
    cpuid
    cmp eax, 0x80000001
    jb lm_support_unavailable
    mov eax, 0x80000001
    cpuid
    test edx, (1 << 29)
    jz lm_support_unavailable
    ret
lm_support_unavailable:
    mov esi, offset msg_no_lm_support
    jmp error32

check_enable_paging:
    # Clear page tables PML4T, PDPT, PDT, and PT.
    mov edi, offset __pml4t_adr
    xor eax, eax
    mov ecx, 0x1000
    rep stosd
    
    # Map memory (512 GiB of virtual memory) onto the page table.
    mov edi, offset __pml4t_adr
    mov eax, offset __pd
    or eax, 3
    mov ecx, 512
.Lfill_pml4t:
    stosd
    add edi, 4
    loop .Lfill_pml4t

    # Map the page directory to the first 512 GiB.
    mov edi, offset __pd
    xor ebx, ebx
    mov ecx, 512
.Lfill_loop:
    mov eax, ebx
    mov edx, ebx
    shl eax, 12
    shr edx, 20
    or eax, 0x83
    mov [edi], eax
    mov [edi+4], edx
    add edi, 8
    add ebx, 0x40000
    loop .Lfill_loop

    # Prevents OoOE shenanigans from screwing us up.
    wbinvd
    mfence

    # Add top page directory address to the cr3.
    mov edi, offset __pml4t_adr
    mov cr3, edi

    # Enable the PEA paging bit.
    mov eax, cr4
    or eax, (1 << 5)
    mov cr4, eax
    
    ret

enter_stage4:
    # Set the LM bit.
    mov ecx, 0xC0000080
    rdmsr
    or eax, (1 << 8)
    wrmsr

    # Enable paging
    mov eax, cr0
    or eax, (1 << 31)
    mov cr0, eax

    lgdt [gdt64_ptr]
    mov eax, offset stage4
    push 0x08
    push eax
    retf

    jmp halt32

msg_no_cpuid_support:   .asciz "No cpuid support for target processor."
msg_no_lm_support:      .asciz "No Long Mode support for target processor."

gdt64_ptr:
    .word gdt64_end - gdt64 - 1
    .quad gdt64 

gdt64:
.L64null:
    .quad 0
.L64code:
    .long 0xFFFF
    .byte 0
    .byte GDT_PRESENT | GDT_NOT_SYS | GDT_EXEC | GDT_RW
    .byte GDT_GRAN_4K | GDT_LONG_MODE | 0xF
    .byte 0
.L64data:
    .long 0xFFFF
    .byte 0
    .byte GDT_PRESENT | GDT_NOT_SYS | GDT_RW
    .byte GDT_GRAN_4K | GDT_SZ_32 | 0xF
    .byte 0
.L64tss:
    .long 0x00000068
    .long 0x00CF8900
    .quad 0
gdt64_end: