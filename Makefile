# Include path settings
include config.mk

# Rust toolchain commands
LLVM_TARGET := thumbv7m-none-eabi.json
RUSTC       := rustc

# GCC toolchain commands
GCC_PREFIX  := arm-none-eabi-
AS          := $(GCC_PREFIX)as
CC          := $(GCC_PREFIX)gcc
CXX         := $(GCC_PREFIX)g++
LD          := $(GCC_PREFIX)ld
GDB         := $(GCC_PREFIX)gdb
OBJCOPY     := $(GCC_PREFIX)objcopy
OBJDUMP     := $(GCC_PREFIX)objdump

# QEMU Tools
QEMU        := QEMU_AUDIO_DRV=none qemu-system-arm
QEMUFLAGS   := -nographic -M lm3s6965evb -m 1M -serial stdio -s

# Directories
BOOT_DIR    := boot
BUILD_DIR   := build

#libcore
EXTRA_DIR 	 := extra
LIBCORE_RLIB := $(EXTRA_DIR)/libcore.rlib

# Armstrong
KERNEL_DIR  := kernel
KERNEL_RLIB := armstrong.rlib

# Flags
RUSTCFLAGS  := -O --target $(LLVM_TARGET) -L $(EXTRA_DIR)
RUSTCFLAGS  += -C target-cpu=cortex-m3 -C soft-float -C relocation-model=static -C code-model=kernel
RUSTCFLAGS  += -C no-redzone -Z no-landing-pads
CPU_FLAGS   := -mcpu=cortex-m3 -mthumb
W_FLAGS     := -Wall -Werror -Wextra -pedantic
CCFLAGS     := $(CPU_FLAGS) $(W_FLAGS) -std=c11
CXXFLAGS    := $(CPU_FLAGS) $(W_FLAGS) -std=c++11
LDFLAGS     := --gc-sections


# App stuff
app         ?= hello
APP_DIR     := apps/$(app)
SOURCES     := $(wildcard $(APP_DIR)/*.rs) $(wildcard $(APP_DIR)/*.c) $(wildcard $(APP_DIR)/*.cpp)
APP_OBJS    := $(foreach file, $(notdir $(SOURCES)), $(BUILD_DIR)/$(basename $(file)).o)

LINK_SCRIPT := $(BOOT_DIR)/linker.ld
OBJS        := $(BUILD_DIR)/loader.o $(APP_OBJS)
BINARY      := $(BUILD_DIR)/$(app).bin

.PHONY: all clean


all: $(BUILD_DIR)/$(app).lst $(BINARY)
	@wc -c $(BINARY)


da: $(BUILD_DIR)/$(app).elf
	$(OBJDUMP) -d $(BUILD_DIR)/$(app).elf

$(BUILD_DIR):
	@mkdir -p $(BUILD_DIR)

clean:
	rm -rf $(BUILD_DIR)

qemu: all
	$(QEMU) $(QEMUFLAGS) -kernel $(BUILD_DIR)/$(app).bin

gdb: all
	$(GDB) $(BUILD_DIR)/$(app).elf

# Keep intermediate files
.SECONDARY:

$(LIBCORE_RLIB): $(RUST_ROOT)/src/libcore/lib.rs | $(EXTRA_DIR)
	$(RUSTC) $(RUSTCFLAGS) $(RUST_ROOT)/src/libcore/lib.rs --out-dir $(EXTRA_DIR)

$(EXTRA_DIR):
	@mkdir -p $(EXTRA_DIR)

# Compile the armstrong kernel
$(BUILD_DIR)/$(KERNEL_RLIB): $(KERNEL_DIR)/*.rs $(LIBCORE_RLIB) | $(BUILD_DIR)
	$(RUSTC) $(RUSTCFLAGS) $(KERNEL_DIR)/lib.rs --out-dir $(BUILD_DIR)

# Compile rust code to object files
$(BUILD_DIR)/%.o: $(APP_DIR)/%.rs $(BUILD_DIR)/$(KERNEL_RLIB) | $(BUILD_DIR)
	$(RUSTC) $(RUSTCFLAGS) --crate-type bin --emit obj $< --out-dir $(BUILD_DIR) -L $(BUILD_DIR) -C lto

# C/C++ code can be compiled directly
$(BUILD_DIR)/%.o: $(APP_DIR)/%.c | $(BUILD_DIR)
	$(CC) -c $(CCFLAGS) -g $< -o $@

$(BUILD_DIR)/%.o: $(APP_DIR)/%.cpp | $(BUILD_DIR)
	$(CXX) -c $(CXXFLAGS) -g $< -o $@

# We also need the boot code
$(BUILD_DIR)/%.o: $(BOOT_DIR)/%.s | $(BUILD_DIR)
	$(AS) $(CPU_FLAGS) -g $< -o $@

# Create the kernel elf file and binary
$(BUILD_DIR)/$(app).elf: $(OBJS) $(LINK_SCRIPT) | $(BUILD_DIR)
	 $(LD) -T $(LINK_SCRIPT) -o $@ $(OBJS) $(LDFLAGS)

$(BUILD_DIR)/%.bin: $(BUILD_DIR)/%.elf | $(BUILD_DIR)
	$(OBJCOPY) -O binary $< $@

$(BUILD_DIR)/$(app).lst: $(BUILD_DIR)/$(app).elf
	$(OBJDUMP) -D $(BUILD_DIR)/$(app).elf > $(BUILD_DIR)/$(app).lst

