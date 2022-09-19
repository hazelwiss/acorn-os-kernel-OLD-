.section .boot.text, "awx"
.code32

.global 
stage3:
    cli # Clear interrupt flag.
    cld # Clear direction flag.

    # Clear bss section for the boot code.
    xor eax, eax
    mov esi, offset __boot_bss_start
    mov ecx, offset __boot_bss_end
    sub ecx, esi
    rep stosb

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
    cli
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

.equ PG1_CNT, 512 * 2
.equ PG2_CNT, 2
check_enable_paging:        
    # Identity map the first two page tables
    
    # Identity map P4
    mov edi, offset __p4
    mov eax, offset __p3
    or eax, 0x003 
    mov [edi], eax

    # Identity map P3
    mov edi, offset __p3
    mov eax, offset __p2
    or eax, 0x003
    mov [edi], eax

    # Map the first 2 GiB of physical memory to virtual memory [0xffffffff80000000, 0xffffffffffffffff].
    
    # Maps the P4.
    mov esi, offset __p4
    mov eax, offset __p3
    or eax, 0x003
    mov [esi + 511 * 8], eax 

    # Maps the P3.
    mov esi, offset __p3
    mov eax, offset __p2
    or eax, 0x003
    mov [esi + 510 * 8], eax
    add eax, 512 * 8
    mov [esi + 511 * 8], eax

    # Maps the P2.
    mov edi, offset __p2
    mov esi, offset __p1
    mov ecx, (512 * PG2_CNT)
    1:
        mov eax, esi
        or eax, 0x003
        stosd
        xor eax, eax
        stosd
        add esi, 512 * 8
        loop 1b

    # Maps the P1.
    mov edi, offset __p1
    mov esi, 0
    mov ecx, (512 * PG1_CNT)
    1:
        mov eax, esi
        or eax, 0x003
        stosd
        xor eax, eax
        stosd
        add esi, 512 * 8
        loop 1b

    # Prevents OoOE shenanigans from screwing us up.
    wbinvd
    mfence

    # Add top page directory address to the cr3.
    mov edi, offset __p4
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
    mov eax, offset jmp_to_right_place
    push 0x08
    push eax
    retf

    jmp halt32

gdt64_ptr:
    .word gdt64_end - gdt64 - 1
    .quad gdt64 

gdt64:
.L64null:
    .quad 0
.L64code:
    .long 0
    .byte 0
    .byte GDT_PRESENT | GDT_NOT_SYS | GDT_EXEC | GDT_RW
    .byte GDT_GRAN_4K | GDT_LONG_MODE | 0xF
    .byte 0
.L64data:
    .long 0
    .byte 0
    .byte GDT_PRESENT | GDT_NOT_SYS | GDT_RW
    .byte GDT_GRAN_4K | GDT_SZ_32 | 0xF
    .byte 0
.L64tss:
    .long 0x00000068
    .long 0x00CF8900
    .quad 0
gdt64_end:

msg_no_cpuid_support:   .asciz "No cpuid support for target processor."
msg_no_lm_support:      .asciz "No Long Mode support for target processor."

.code64
.section .boot.text

jmp_to_right_place:
    mov rax, offset stage4
    jmp rax
loop64:
    cli 
    hlt
    jmp loop64
