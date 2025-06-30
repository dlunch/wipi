#!/usr/bin/env bash

cargo -Zbuild-std=core build -p examples --examples --target thumbv4t-none-eabi

cargo run -p wipi-archiver -- target/thumbv4t-none-eabi/debug/examples/helloworld Clet 00000000 PD000000 > target/helloworld.zip
