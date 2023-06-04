#!/bin/bash

#cargo build --release --target x86_64-unknown-linux-musl
#cargo build --target x86_64-unknown-linux-musl
#cross build --release
cargo build

mkdir -p ../bin
#cp --preserve=mode ./target/x86_64-unknown-linux-musl/release/fantastic-rs ../bin/backend
#cp --preserve=mode ./target/x86_64-unknown-linux-musl/debug/fantastic-rs ../bin/backend
#cp --preserve=mode ./target/release/fantastic-rs ../bin/backend
cp --preserve=mode ./target/debug/fantastic-rs ../bin/backend

