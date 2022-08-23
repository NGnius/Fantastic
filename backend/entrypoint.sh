#!/bin/sh
set -e

echo "Container's IP address: `awk 'END{print $1}' /etc/hosts`"

cd /backend

$HOME/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/cargo build --release
mkdir -p out
cp target/release/fantastic-rs out/backend