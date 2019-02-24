QEMU_FLAGS += -serial mon:stdio
QEMU_FLAGS += -device isa-debug-exit,iobase=0xf4,iosize=0x04
QEMU_FLAGS += -display none

ifneq ($(OS),Windows_NT)
    UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Darwin)
        QEMU_FLAGS += -L $(shell echo "$(shell dirname $(shell which qemu-system-x86_64))/../pc-bios")
    endif
endif

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
