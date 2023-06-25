#!/bin/sh

# Install Wasm-Pack
# cargo install wasm-pack

# Build Catgirl Engine
wasm-pack build --target web --no-pack --no-typescript --out-name main -d `pwd` --release

# Start Server
python3 start.py