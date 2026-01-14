#!/usr/bin/env bash

examples=$(ls examples/src | grep .rs | sed 's/\.rs//')

cargo -Zbuild-std=core,alloc build -p examples --target thumbv4t-none-eabi --features ktf --profile examples --no-default-features
for example in $examples
do
    cargo run -p wipi_archiver -- ktf target/thumbv4t-none-eabi/examples/$example Clet 00000000 PD000000 ./examples/resources/${example} > target/${example}_ktf.zip
done

cargo -Zbuild-std=core,alloc build -p examples --target thumbv4t-none-eabi --features lgt --profile examples --no-default-features
for example in $examples
do
    cargo run -p wipi_archiver -- lgt target/thumbv4t-none-eabi/examples/$example Clet 00000000 PD000000 ./examples/resources/${example} > target/${example}_lgt.zip
done