global real_mode_start
extern kmain

%include "paging.asm"
%include "gdt.asm"

; Real mode text section
section .text
bits 32

; Real mode entry point
real_mode_start:

  ; Point stack pointer to stack
  mov esp, stack_top

  ; Set paging up
  call setup_paging

  ; Load the global descriptor table
  lgdt [gdt64.pointer]

  ; Jump into long mode
  jmp enter_long_mode

section .bss
align 4096

; Bootstrap stack
stack_bottom:
  resb 256
stack_top:
