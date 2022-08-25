DEBUG_OUT := target/amd64-kernel/debug/kernel
SRC_ROOT = $(abspath .)

.PHONY: all 
all: qemu

.PHONY: build_debug
build_debug:
	@cargo build --target ${SRC_ROOT}/targets/amd64/amd64-kernel.json
	@cp ${DEBUG_OUT} ${DEBUG_OUT}.not_stripped
	@objcopy ${DEBUG_OUT} -O binary

.PHONY: qemu
qemu: build_debug 
	qemu-system-x86_64 -drive format=raw,file=${DEBUG_OUT} -s -S --no-reboot -monitor stdio -d in_asm -m 512

.PHONY: clean
clean:
	cargo clean