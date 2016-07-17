section .multiboot
mb_start:

  ; Defines
  MB_MAGIC    equ 0xe85250d6
  MB_ARCH     equ 0
  MB_LENGTH   equ mb_end - mb_start
  MB_CHECKSUM equ 0x100000000 - (MB_MAGIC + MB_ARCH + (mb_end - mb_start))
  MB_TYPE     equ 0
  MB_FLAGS    equ 0
  MB_SIZE     equ 8

  ; Memory layout
  dd MB_MAGIC
  dd MB_ARCH
  dd MB_LENGTH
  dd MB_CHECKSUM
  dw MB_TYPE
  dw MB_FLAGS
  dd MB_SIZE
mb_end:
