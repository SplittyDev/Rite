section .text
bits 32

; Set the GDT up
enter_long_mode:

  ; Update segment registers
  mov ax, gdt64.data
  mov ss, ax
  mov ds, ax
  mov es, ax

  ; Far jump into long mode
  jmp gdt64.code:long_mode_start

section .rodata

; Defines
GDT_BIT_READWRITE   equ 41
GDT_BIT_EXECUTABLE  equ 43
GDT_BIT_TYPE        equ 44
GDT_BIT_PRESENT     equ 47
GDT_BIT_LONG        equ 53

; Global Descriptor Table
gdt64:

  ; Zero segment
  .zero equ $ - gdt64:
    dq 0

  ; Code segment
  .code: equ $ - gdt64
    dq \
      (1 << GDT_BIT_TYPE)       |\
      (1 << GDT_BIT_PRESENT)    |\
      (1 << GDT_BIT_READWRITE)  |\
      (1 << GDT_BIT_EXECUTABLE) |\
      (1 << GDT_BIT_LONG)

  ; Data segment
  .data: equ $ - gdt64
    dq \
      (1 << GDT_BIT_TYPE)     |\
      (1 << GDT_BIT_PRESENT)  |\
      (1 << GDT_BIT_READWRITE)

  ; GDT pointer
  .pointer:
    dw .pointer - gdt64 - 1
    dq gdt64
