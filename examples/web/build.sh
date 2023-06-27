#!/bin/sh

# Install Wasm-Pack
# cargo install wasm-pack
# cargo install wasm-opt
# cargo install wasm-bindgen-cli
# cargo install twiggy

# Build Catgirl Engine
# wasm-pack build --target no-modules --no-pack --no-typescript --out-name main --out-dir "`pwd`" --release
cargo build --target wasm32-unknown-unknown --release --lib -Z build-std=panic_abort,std
wasm-bindgen --target no-modules --no-typescript --keep-debug --out-dir "`pwd`" --out-name main "`pwd`/../../target/wasm32-unknown-unknown/release/main.wasm"
wasm-opt -Oz "`pwd`/main_bg.wasm" --enable-threads --enable-mutable-globals --enable-bulk-memory --enable-exception-handling --output "`pwd`/main.wasm"
wasm-opt -Oz "`pwd`/main_bg.wasm" --strip-debug --enable-threads --enable-mutable-globals --enable-bulk-memory --enable-exception-handling --output "`pwd`/main_no_debug.wasm"

rm "`pwd`/main_bg.wasm"

# Start Server
python3 start.py