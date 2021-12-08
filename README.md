# Rust in NEORV32 - Work in Progress (WIP)
Compiling and running an embedded Rust application for the NEORV32 RISC-V soft core CPU

# Hardware

Current development is based on the [Arduino MKR Vidor 4000 FPGA Board](https://store.arduino.cc/products/arduino-mkr-vidor-4000)
# Building

First of all, source the NEORV32 RISCV tools. If in doubt, check the [NEORV32 Software Toolchain Setup](https://stnolting.github.io/neorv32/ug/#_software_toolchain_setup).

In order to generate the image that is downloaded by UART, the image_bin binary needs to be compiled first. 
Simplest way currently is compiling a software example:

    cd submodules/neorv32/example/blink_led
    make exe

The current *testing* application can be built with the following script:

    ./generate_bin.sh

**NOTE: This will build and image for UART uploading via the bootloader.**

The generated NEORV32 binary in located under bin folder. You can proceed with the programming procedure as explained in the [NEORV32 User Guide](https://stnolting.github.io/neorv32/ug/#_uploading_and_starting_of_a_binary_executable_image_via_uart)

# Current Issues and ToDos
- There is an issue with the UART, the program hangs when there are multiple print statements, this does not happens when the program is loaded by the debugger
- Current memory definitions are/might not be accurate. It assumes that the NEORV32 uses the SDRAM available on the Vidor 4000 with a larger size than actually required
# Acknowledgments

- Initial code based on the developments for the [iCEBreaker-FPGA](https://github.com/icebreaker-fpga/icebreaker-litex-examples/tree/master/r-riscv-blink)
- Very useful video on [Writting a Rust HAL from Scratch](https://www.youtube.com/watch?v=pj2Rk-ftcWA) from Jame's Office Hours
- *To be updated*