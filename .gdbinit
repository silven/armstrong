define rt
  target remote localhost:1234
  load
  break c_entry
  layout asm
  layout regs
end
