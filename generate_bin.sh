#!/bin/bash
cargo build

# NEORV32 binary name
NEORV32_BINARY="neorv32"

# Output folder
OUTDIR="bin"

if [ ! -d "$OUTDIR" ]; then
  echo "Creating output folder"
  mkdir bin
fi

echo "Extracting sections from Rust's ELF"
riscv32-unknown-elf-objdump -d -S -z target/riscv32i-unknown-none-elf/debug/neorv32-rust > $OUTDIR/neorv32-rust.asm
riscv32-unknown-elf-objcopy -I elf32-little target/riscv32i-unknown-none-elf/debug/neorv32-rust -j .text -O binary $OUTDIR/text.bin
riscv32-unknown-elf-objcopy -I elf32-little target/riscv32i-unknown-none-elf/debug/neorv32-rust -j .rodata -O binary $OUTDIR/rodata.bin
riscv32-unknown-elf-objcopy -I elf32-little target/riscv32i-unknown-none-elf/debug/neorv32-rust -j .data   -O binary $OUTDIR/data.bin
cat $OUTDIR/text.bin $OUTDIR/rodata.bin $OUTDIR/data.bin > $OUTDIR/main.bin

echo "Generating NEORV32 binary"
submodules/neorv32/sw/image_gen/image_gen -app_bin $OUTDIR/main.bin $OUTDIR/$NEORV32_BINARY.bin

echo "Done!"