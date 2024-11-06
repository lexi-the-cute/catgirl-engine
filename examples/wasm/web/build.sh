#!/bin/sh
HOST=http://127.0.0.1:8000/pkg
# RUST_LOG=info
CURRENT_DIR=`pwd`
cd ../../..

cp -a `pwd`/License.md `pwd`/LICENSE

# Generates usable Wasm binary and supporting files
wasm-pack build --profiling --target web -d $CURRENT_DIR/pkg

# Creates Wasm Source Map to aid in debugging
cargo wasm2map $CURRENT_DIR/pkg/main_bg.wasm --patch --base-url $HOST

# cargo build --target wasm32-unknown-unknown
# wasm-bindgen --keep-debug --web --out-dir pkg ./target/wasm32-unknown-unknown/

rm `pwd`/LICENSE
