MEMORY {
	FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 0x8000
	RAM (wail) : ORIGIN = 0x10000000, LENGTH = 0x2000
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);
_boot_checksum = 0 - (_stack_start + reset_handler + 1 + (hang + 1) * 5);

ENTRY(reset_handler)

__aeabi_unwind_cpp_pr0 = hang;
__aeabi_memclr4 = __aeabi_memclr;

SECTIONS {
  .text : {
    KEEP(*(.isr_vector))

 	*(.text*)
  } > FLASH

  .bss : {
 	* (.bss*)
  } > RAM
}
