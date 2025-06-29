#!/usr/bin/env bash

cargo -Zunstable-options -C examples build --examples

cargo run -p wipi-archiver -- target/thumbv4t-none-eabi/debug/examples/helloworld Clet 00000000 PD000000 > helloworld.zip
