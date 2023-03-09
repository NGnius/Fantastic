#!/bin/bash

echo "--- Rust version info ---"
rustup --version
rustc --version
cargo --version

echo "--- Building plugin backend ---"
cargo build --release --features usdpl-back/decky
mkdir -p out
cp target/release/fantastic-rs out/backend

echo " --- Cleaning up ---"
# remove root-owned target folder
cargo clean
