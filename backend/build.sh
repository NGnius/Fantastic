#!/bin/bash

cross build --release
mkdir out
cp ./target/release/fantastic-rs out/backend
