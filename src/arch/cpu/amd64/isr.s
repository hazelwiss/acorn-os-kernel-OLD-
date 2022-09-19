.code64
.section .text

.macro PUSH_REGS
	push rax
	push rbx
	push rcx
	push rdx
	push rsi
	push rdi
	push r8
	push r9
	push r10
	push r11
	push r12
	push r13
	push r14
	push r15
	push rbp
.endm

.macro POP_REGS
	pop rbp
	pop r15
	pop r14
	pop r13
	pop r12
	pop r11
	pop r10
	pop r9
	pop r8
	pop rdi
	pop rsi
	pop rdx
	pop rcx
	pop rbx
	pop rax
.endm

.macro EXCEPT id
	.align 8
	.global __except_handler\id\()
	__except_handler\id\():
		push \id\() 
		PUSH_REGS
		cld
		mov rdi, rsp
		.extern except
		call except
		mov rsp, rax
		POP_REGS
		add rsp, 16
		iretq
.endm

EXCEPT 0
EXCEPT 1
EXCEPT 2
EXCEPT 3
EXCEPT 4
EXCEPT 5
EXCEPT 6
EXCEPT 7
EXCEPT 8
EXCEPT 9
EXCEPT 10
EXCEPT 11
EXCEPT 12
EXCEPT 13
EXCEPT 14
EXCEPT 15
EXCEPT 16 
EXCEPT 17
EXCEPT 18
EXCEPT 19
EXCEPT 20 
EXCEPT 21 
EXCEPT 22
EXCEPT 23
EXCEPT 24
EXCEPT 25
EXCEPT 26 
EXCEPT 27
EXCEPT 28
EXCEPT 29
EXCEPT 30
EXCEPT 31

.macro IRQ id handler
	.align 8
	.global __irq_handler\id\()
	__irq_handler\id\():
		push \id\()
		PUSH_REGS
		cld
		mov rdi, rsp
		.extern \handler\()
		call \handler\()
		mov rsp, rax
		POP_REGS
		add rsp, 8
		iretq
.endm

IRQ 0 piti # Programmable interrupt timer
IRQ 1 kbd # keyboard
IRQ 2 cascade # unused
IRQ 3 com2 # serial
IRQ 4 com1 # serial
IRQ 5 lpt2 # ?
IRQ 6 floppy # floppy disk 
IRQ 7 lpt1 # ?
IRQ 8 rtc # real-time-clock
IRQ 9 free0
IRQ 10 free1 
IRQ 11 free2 
IRQ 12 mouse # ps2 mouse
IRQ 13 cp # fpu/co-processor/inter-processor 
IRQ 14 pata # primary ATA disk 
IRQ 15 sata # secondary ATA disk 
