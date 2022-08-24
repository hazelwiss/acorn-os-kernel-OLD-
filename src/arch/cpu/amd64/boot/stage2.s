.section .boot, "awx"
.code16

.global stage2
stage2:
    # Load memory map into buffer.
    # TODO!

    # Load rest of the bootloader into memory. Disk is divided into 512 byte blocks.
    mov cx, 1 
    mov lba_packet_size, cx 
    mov esi, offset __disk_load_buffer
    mov lba_load_buffer, esi
    mov si, offset msg_bl_zs
    jz error
    mov edi, offset __kernel_load_adr
    mov bx, lba_starting_lba
    add bx, lba_packet_size
    inc bx
    mov ecx, offset __kernel_load_size
    shr ecx, 9 # Get the amount of blocks to load from disk by dividing byte size by 512. (assuming alignment).
.Lload_from_disk:
    push ecx
    push edi
 
    # Setup LBA disk loading interrupt to load a sector into memory to copy over.
    mov lba_starting_lba, bx
    inc bx
    mov si, offset lba_struct
    mov ah, 0x42
    mov dl, 0x80
    int 0x13

    # Copy the recently loaded sector from the buffer into relevant memory.
    pop edi
    mov esi, offset __disk_load_buffer
    mov ecx, 512
    rep movsb [edi], [esi]
    pop ecx
    loop .Lload_from_disk

    # Turn off cursor for the framebuffer.
    mov ah, 1
    mov ch, 0x3F
    int 0x10

    # Enter protected mode for real this time and enter stage 3!
    lgdt[gdt32_ptr] 
    mov eax, cr0
    or ax, 1 
    mov cr0, eax

    # Set segment registers for PM.
    mov bx, 0x10 
    mov ds, bx 
    mov ss, bx 
    mov es, bx 
    mov gs, bx 

    # Make a far return to enter into PM at stage3 address.
    mov eax, offset stage3
    push 0x08
    push eax
    retf 

    mov si, msg_halt
    jmp error

