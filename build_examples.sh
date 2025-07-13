#!/usr/bin/env bash

cargo -Zbuild-std=core build -p examples --target thumbv4t-none-eabi --features ktf

cargo run -p wipi_archiver -- ktf target/thumbv4t-none-eabi/debug/helloworld Clet 00000000 PD000000 > target/helloworld_ktf.zip

cargo -Zbuild-std=core build -p examples --target thumbv4t-none-eabi --features lgt

cargo run -p wipi_archiver -- lgt target/thumbv4t-none-eabi/debug/helloworld Clet 00000000 PD000000 > target/helloworld_lgt.zip