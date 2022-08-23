#!/bin/bash

cross build --release
mkdir ../bin
cp ./target/release/fantastic-rs ../bin/backend
