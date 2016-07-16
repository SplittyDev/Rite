ENTRY(start)

SECTIONS {
  . = 1M;

  .boot : {
    *(.multiboot)
  }

  .text : {
    *(.text)
  }

  .bss : ALIGN(4096) {
    *(.bss)
  }
}
