#!/usr/bin/env bash

function ri_setup {
  # Rite ISO directory
  export ri_isodir="iso"
  # Rite ISO boot directory
  export ri_bootdir="iso/boot"
  # Rite assembly directory
  export ri_asmdir="src/asm"
}

function ri_assemble {
  pushd $ri_asmdir &>/dev/null
  printf "Assembling (\n"
  fail=false
  for fasm in $*; do
    printf "  [ => ] $fasm"
    if ! nasm -felf64 $fasm &>/dev/null; then
      fail=true
      printf "\r  [FAIL] $fasm"
    else
      printf "\r  [ OK ] $fasm"
    fi
    printf "\n"
  done
  printf ") "
  if $fail; then
    printf "[FAIL]\n"
    exit 1
  fi
  printf "[ OK ]\n"
  popd &>/dev/null
}

function ri_link {
  printf "Linking (\n"
  for fobj in $*; do
    mv "$ri_asmdir/$fobj" "./"
    printf "  [ ?? ] $fobj\n"
  done
  printf ") "
  if ! ld --nmagic --output=kernel.elf --script=linker.ld $*; then
    printf "[FAIL]\n"
    exit 2
  fi
  printf "[ OK ]\n"
}

function ri_verify-multiboot2 {
  printf "Verifying multiboot2 header... "
  if grub-file --is-x86-multiboot2 kernel.elf; then
    printf "OK\n"
  else
    printf "FAIL\n"
    exit 3
  fi
}

function ri_build-iso {
  printf "Creating Rite ISO image... "
  cp kernel.elf "$ri_bootdir"
  if grub-mkrescue -o rite.iso "$ri_isodir" &>/dev/null; then
    printf "OK\n"
  else
    printf "FAIL\n"
    exit 4
  fi
}

function ri_post-cleanup {
  rm *.o &>/dev/null
  rm trace-* &>/dev/null
}

function compile {
  ri_setup
  ri_assemble \
    "multiboot.asm" \
    "boot.asm"
  ri_link \
    "multiboot.o" \
    "boot.o"
  ri_verify-multiboot2
  ri_build-iso
  ri_post-cleanup
}

compile
exit 0
