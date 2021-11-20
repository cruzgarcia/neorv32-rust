 # Sourcing NEORV32
 # Run this in /media/labshd/Projects/neorv32_rust
#. neorv32
cargo build

riscv32-unknown-elf-objdump -d -S -z target/riscv32i-unknown-none-elf/debug/neorv32-rust > exec/neorv32-rust.asm
riscv32-unknown-elf-objcopy -I elf32-little target/riscv32i-unknown-none-elf/debug/neorv32-rust -j .text -O binary exec/text.bin
riscv32-unknown-elf-objcopy -I elf32-little target/riscv32i-unknown-none-elf/debug/neorv32-rust -j .rodata -O binary exec/rodata.bin
riscv32-unknown-elf-objcopy -I elf32-little target/riscv32i-unknown-none-elf/debug/neorv32-rust -j .data   -O binary exec/data.bin
cat exec/text.bin exec/rodata.bin exec/data.bin > exec/main.bin

# This has to be run in /media/labshd/Projects/neorv32-mkr-vidor-4000/submodules/neorv32/sw/image_gen
#./image_gen -app_bin /media/labshd/Projects/neorv32_rust/main.bin  /media/labshd/Projects/neorv32_rust/main_for_fpga_neorv32.bin
/media/labshd/Projects/neorv32-mkr-vidor-4000/submodules/neorv32/sw/image_gen/image_gen -app_bin exec/main.bin exec/main_for_fpga_neorv32.bin
