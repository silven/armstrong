BUILD_DIR   := target/thumbv7m-none-eabi/release

app         ?= hi_rust

# Flags
RUSTCFLAGS  := -C soft-float -C code-model=kernel -Z no-landing-pads
LDFLAGS     := --gc-sections

LINK_SCRIPT := $(BOOT_DIR)/linker.ld

.PHONY: all clean armstrong

all: $(BUILD_DIR)/$(app).bin $(BUILD_DIR)/$(app).lst
	@wc -c $<

clean:
	cargo clean

test:
	cargo test --verbose

# Compile the wanted app
$(BUILD_DIR)/$(app): armstrong
	cargo build --verbose --features kernel_mode --bin $(app) --release --target thumbv7m-none-eabi

# Compile the armstrong kernel
armstrong:
	cargo build --verbose --features kernel_mode --lib --release --target thumbv7m-none-eabi

# Objdump into a binary
$(BUILD_DIR)/$(app).bin: $(BUILD_DIR)/$(app)
	arm-none-eabi-objcopy -O binary $< $@

# Disassemble
$(BUILD_DIR)/$(app).lst: $(BUILD_DIR)/$(app)
	arm-none-eabi-objdump -D $< > $@

