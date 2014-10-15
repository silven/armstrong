.cpu cortex-m3
.thumb
.syntax unified

.global _isr_vector
_isr_vector:
.word   _stack_start      // start of the stack
.word   main              // reset handler
.word   hang              // NMI handler
.word   hang              // hard fault handler
.word   hang              // MPU fault handler
.word   hang              // bus fault handler
.word   hang              // usage fault handler
.word   _boot_checksum    // boot checksum?
.word   hang              // reserved
.word   hang              // reserved
.word   hang              // reserved
.word   hang              // SV call handler
.word   hang              // debug monitor handler
.word   hang              // reserved
.word   hang              // PendSV handler
.word   hang              // SysTick handler
.word   hang              // No idea what this is for


.thumb_func
.global hang
hang:
  B .
