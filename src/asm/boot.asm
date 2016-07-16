global start

section .text
bits 32

start:
  call setup_paging
  lgdt [gdt64.pointer]
  mov word [0xb8000], 0x0248 ; H
  mov word [0xb8002], 0x0265 ; e
  mov word [0xb8004], 0x026c ; l
  mov word [0xb8006], 0x026c ; l
  mov word [0xb8008], 0x026f ; o
  mov word [0xb800a], 0x022c ; ,
  mov word [0xb800c], 0x0220 ;
  mov word [0xb800e], 0x0277 ; w
  mov word [0xb8010], 0x026f ; o
  mov word [0xb8012], 0x0272 ; r
  mov word [0xb8014], 0x026c ; l
  mov word [0xb8016], 0x0264 ; d
  mov word [0xb8018], 0x0221 ; !
  hlt

setup_paging:
  ; Point PML4 to PDP
  mov eax, page_directory_pointer_table
  or eax, 0b11
  mov dword [page_map_level4_table], eax
  ; Point PDP to PD
  mov eax, page_directory_table
  or eax, 0b11
  mov dword [page_directory_pointer_table], eax
  ; Initialize counter
  mov ecx, 0
  ; Point all page directories to a page
  .map_page_directory_table:
    mov eax, 0x200000
    mul ecx
    or eax, 0b10000011
    mov [page_directory_table + ecx * 8], eax
    inc ecx
    cmp ecx, 512
    jne .map_page_directory_table
  ; Move page table to cr3
  mov eax, page_map_level4_table
  mov cr3, eax
  ; Enable physical address extension
  mov eax, cr4
  or eax, 1 << 5
  mov cr4, eax
  ; Set long mode bit
  mov ecx, 0xC0000080
  rdmsr
  or eax, 1 << 8
  wrmsr
  ; Enable paging
  mov eax, cr0
  or eax, 1 << 31
  or eax, 1 << 16
  mov cr0, eax
  ret

section .bss
page_map_level4_table:
  resb 4096
page_directory_pointer_table:
  resb 4096
page_directory_table:
  resb 4096

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
