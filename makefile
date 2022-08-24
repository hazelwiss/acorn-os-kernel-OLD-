DEBUG_OUT := target/amd64-kernel/debug/kernel

all: qemu

build_debug:
	cargo build --target targets/amd64/amd64-kernel.json
	cp ${DEBUG_OUT} ${DEBUG_OUT}.not_stripped
	objcopy ${DEBUG_OUT} -O binary

qemu: build_debug 
	qemu-system-x86_64 -drive format=raw,file=${DEBUG_OUT} -s -S --no-reboot -monitor stdio -d in_asm -m 512

clean:
	cargo clean