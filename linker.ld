ENTRY(_start);

MEMORY
{
  RAM : ORIGIN = 0x80000000, LENGTH = 128K
}

SECTIONS
{
  .text : {
    KEEP(*(.text._start))
    *(.text*)
  } > RAM

  .rodata : { *(.rodata*) } > RAM
  .data : { *(.data*) } > RAM
  .bss : {
    *(.bss*)
    *(.bss.stacks)
    *(COMMON)
  } > RAM
}
