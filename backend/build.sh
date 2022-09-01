#!/bin/bash

cargo build --target x86_64-unknown-linux-musl --release
mkdir -p ../bin
cp ./target/x86_64-unknown-linux-musl/release/fantastic-rs ../bin/backend
