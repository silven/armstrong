BUILD_DIR   := target/thumbv7m-none-eabi/release
MBED_DIR    := /Volumes/MBED
MBED_TTY    := /dev/tty.usbmodem1412

app         ?= hi_rust

# Flags
RUSTCFLAGS  := -C soft-float -C code-model=kernel -Z no-landing-pads
LDFLAGS     := --gc-sections

LINK_SCRIPT := $(BOOT_DIR)/linker.ld

all: $(BUILD_DIR)/$(app).bin $(BUILD_DIR)/$(app).lst
	@wc -c $<
da: $(BUILD_DIR)/$(app).lst
	cat $<
clean:
	cargo clean
test:
	cargo test --verbose
flash: $(BUILD_DIR)/$(app).bin
	cp $(BUILD_DIR)/$(app).bin $(MBED_DIR)/$(app).bin
	python -c "import termios; import os; termios.tcsendbreak(os.open('$(MBED_TTY)', os.O_WRONLY | os.O_NONBLOCK), 0)"

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

.PHONY: all clean flash armstrong
