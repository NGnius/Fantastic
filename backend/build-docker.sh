#!/bin/bash

$HOME/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/cargo build --release
mkdir -p out
cp target/release/fantastic-rs out/backend