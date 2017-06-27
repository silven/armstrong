MEMORY {
	FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 0x8000
	RAM (wail) : ORIGIN = 0x10000000, LENGTH = 0x2000
}

__stack_start = ORIGIN(RAM) + LENGTH(RAM);
__boot_checksum = 0 - (__stack_start + reset_handler + 1 + (abort + 1) * 5);

__data_load = LOADADDR(.data);

ENTRY(reset_handler)

SECTIONS {
  .text : ALIGN(4) {
    KEEP(*(.isr_vector))
 	  * (.text*)
  } > FLASH

  .rodata : ALIGN(4) {
 	  * (.rodata*)
  } > FLASH

  .data : ALIGN(4) {
    __data_start = .;
    * (.data*)
    . = ALIGN(4);
    __data_end = .;
  } > RAM AT>FLASH

  .bss : ALIGN(4) {
    __bss_start = .;
 	  * (.bss*)
    __bss_end = .;
  } > RAM

}
