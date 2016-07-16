#!/usr/bin/env bash

function ri_setup {
  # Rite kernel name
  export ri_kernel="rite"
  # Rite ISO directory
  export ri_isodir="iso"
  # Rite ISO boot directory
  export ri_bootdir="iso/boot"
  # Rite assembly directory
  export ri_asmdir="src/asm"
  # Rite Cargo target
  export ri_target_triple="x86_64-unknown-rite-gnu"
  # Rust libcore location
  export ri_libcore="libcore"
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
    mv "$ri_asmdir/$fobj" "./" &>/dev/null
    printf "  [ ?? ] $fobj\n"
  done
  printf ") "
  cp "target/$ri_target_triple/release/lib$ri_kernel.a" "./" &>/dev/null
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

function ri_build-kernel {
  printf "Compiling kernel... "
  export RUSTFLAGS="-L $ri_libcore/target/$ri_target_triple/release"
  if ! cargo build --release \
    --target $ri_target_triple.json &>/dev/null; then
    printf "FAIL\n"
    exit 5
  fi
  printf "OK\n"
}

function ri_build-iso {
  printf "Creating $ri_kernel ISO image... "
  cp kernel.elf "$ri_bootdir"
  if grub-mkrescue -o $ri_kernel.iso "$ri_isodir" &>/dev/null; then
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
  ri_build-kernel
  ri_link \
    "multiboot.o" \
    "boot.o" \
    "lib$ri_kernel.a"
  ri_verify-multiboot2
  ri_build-iso
  ri_post-cleanup
}

compile
exit 0
