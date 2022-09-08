# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/sakana-os
KERNEL_BIN := $(KERNEL_ELF).bin
DISASM_TMP := target/$(TARGET)/$(MODE)/asm

# Building mode argument
ifeq ($(MODE), release)
	MODE_ARG := --release
endif

# BOARD
BOARD ?= qemu
SBI ?= rustsbi
BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin

# Entry path
ifeq ($(BOARD), qemu)
	KERNEL_ENTRY_PA := 0x80200000
endif

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# Make sure env right
env:
	@cargo install cargo-binutils
	@rustup component add llvm-tools-preview rustfmt
	@rustup target add $(TARGET)

# Build OS binary and drop metadate
build: env
	@echo Platform: $(BOARD)
	@cargo build $(MODE_ARG)
	@rust-objcopy --strip-all $(KERNEL_ELF) \
	-O binary $(KERNEL_BIN)

run: build-usr run-inner

run-inner: build
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \

debug: build
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \
		-s -S

build-usr:
	cd ../usr; make run; cd ../os

gdb:
	riscv64-unknown-elf-gdb \
    -ex 'file $(KERNEL_ELF)' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'