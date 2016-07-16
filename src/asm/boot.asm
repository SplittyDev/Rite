global start

section .text
bits 32
start:
  call setup_paging
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
align 4096

; Paging stuff
page_map_level4_table:
  resb 4096
page_directory_pointer_table:
  resb 4096
page_directory_table:
  resb 4096
