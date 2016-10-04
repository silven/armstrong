BUILD_DIR   := target/thumbv7m-none-eabi/release
MBED_DIR    := /media/mikael/MBED
MBED_TTY    := /dev/ttyACM0

CARGO_FLAGS := --verbose --release --target thumbv7m-none-eabi

LINK_SCRIPT := $(BOOT_DIR)/linker.ld

app         ?= hi_rust

all: $(BUILD_DIR)/$(app) $(BUILD_DIR)/$(app).bin $(BUILD_DIR)/$(app).lst
	arm-none-eabi-size $<
da: $(BUILD_DIR)/$(app).lst
	cat $<
clean:
	cargo clean
test:
	cargo test --verbose
flash: $(BUILD_DIR)/$(app).bin
	cp $(BUILD_DIR)/$(app).bin $(MBED_DIR)/$(app).bin

# Compile the wanted app
$(BUILD_DIR)/$(app): armstrong
	cargo build $(CARGO_FLAGS) --bin $(app)

# Compile the armstrong kernel
armstrong:
	cargo build $(CARGO_FLAGS) --lib

# Objdump into a binary
$(BUILD_DIR)/$(app).bin: $(BUILD_DIR)/$(app)
	arm-none-eabi-objcopy -O binary $< $@

# Disassemble
$(BUILD_DIR)/$(app).lst: $(BUILD_DIR)/$(app)
	arm-none-eabi-objdump -D $< > $@

.PHONY: all clean flash armstrong
