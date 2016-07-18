global real_mode_start
extern kmain_setup
extern kmain

%include "paging.asm"
%include "gdt.asm"

; Real mode text section
section .text
bits 32

; Real mode entry point
real_mode_start:

  ; Disable interrupts
  cli

  ; Point stack pointer to stack
  mov esp, stack_top

  ; Load the multiboot info pointer
  mov edi, ebx

  ; Set paging up
  call setup_paging

  ; Load the global descriptor table
  lgdt [gdt64.pointer]

  ; Jump into long mode
  jmp enter_long_mode

; Long mode text section
section .text
bits 64

; Long mode entry point
long_mode_start:

  ; Call the kernel setup
  call kmain_setup

  ; Jumo into the kernel
  jmp kmain

  ; Disable interrupts and halt
  cli
  hlt

section .bss
align 4096

; Bootstrap stack
stack_bottom:
  resb 4096
stack_top:
