ENTRY(_isr_vector)

MEMORY {
	FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 0x8000
	RAM (wail) : ORIGIN = 0x10000000, LENGTH = 0x2000
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);
_boot_checksum = 0 - (_stack_start + main + 1 + (hang + 1) * 5);

SECTIONS {
 .text : {
    KEEP(*(_isr_vector));
 	* (.text*)
 } > FLASH

 .bss : {
 	* (.bss*)
 } > RAM

}
