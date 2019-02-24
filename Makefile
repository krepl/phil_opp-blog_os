ifneq ($(OS),Windows_NT)
    UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Darwin)
        BOOTIMAGE_RUN_FLAGS += -L $(shell echo "$(shell dirname $(shell which qemu-system-x86_64))/../pc-bios")
    endif
endif

all:
	bootimage build

lint:
	cargo clippy

test:
	cargo test && bootimage test --verbose

emu:
	bootimage run -- $(BOOTIMAGE_RUN_FLAGS)

clean:
	cargo clean
