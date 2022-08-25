.section .setup, "awx"
.code64

.global stage4
stage4:
    # Set segment registers for LM.
    mov ax, 0x18
    mov ds, bx 
    mov ss, bx 
    mov es, bx 
    mov gs, bx 

    # Set new stack for long mode. (~14MiB)
    mov rsp, offset __kernel_stack_top

    # Make sure interrupts are disabled.
    cli 

    # Make sure the direction flag is cleared
    cld

    # Enter into Rust code.
    jmp kmain
looping:
    jmp looping