#!/usr/bin/env bash
cargo build

pushd examples >/dev/null
cargo build --examples
popd >/dev/null

target/debug/wipi-archiver examples/target/thumbv4t-none-eabi/debug/examples/helloworld Clet 00000000 PD000000 > helloworld.zip