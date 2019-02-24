QEMU_FLAGS += -serial mon:stdio
QEMU_FLAGS += -device isa-debug-exit,iobase=0xf4,iosize=0x04
QEMU_FLAGS += -display none

all:
	bootimage build

lint:
	cargo clippy

test:
	cargo test && bootimage test --verbose

run:
	bootimage run -- $(QEMU_FLAGS)

clean:
	cargo clean
