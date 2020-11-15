#!/bin/bash

echo "> installing cargo-script"
cargo install cargo-script

echo "> removing old directories"
rm -rf src/stubs
mkdir src/stubs

echo "> cloning fresh protos..."
rm -rf proto
git clone https://github.com/pylonbot/pylon-gateway-protobuf.git proto

echo "> generating protos"
cargo script update_stubs.rs

echo "> building"
cargo build