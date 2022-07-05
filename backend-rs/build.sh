#!/bin/bash

cross build --release
cp ./target/release/fantastic-rs ../backend
