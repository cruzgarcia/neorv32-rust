#!/bin/bash
echo "Starting openocd server"
openocd -f ../submodules/neorv32/sw/openocd/openocd_neorv32.cfg
