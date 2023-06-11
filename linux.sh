#!/bin/bash

rustup target add x86_64-unknown-linux-gnu
rm -rf "./linux/gnu"
TARGET_CC=x86_64-unknown-linux-gnu cargo build --bin json_tester --target=x86_64-unknown-linux-gnu --release --target-dir "./linux/gnu"

sudo apt-get update
sudo apt-get install musl-tools -y
rustup target add x86_64-unknown-linux-musl
rm -rf "./linux/musl"
cargo build --bin json_tester --target=x86_64-unknown-linux-musl --release --target-dir "./linux/musl"