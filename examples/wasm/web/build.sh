#!/bin/sh
PROFILE="debug"  # "debug" or "release"
HOST=http://127.0.0.1:8000/pkg
# RUST_LOG=info

# Build Time Autovars
SCRIPT=`realpath "$0"`
SCRIPT_DIR=`dirname "$SCRIPT"`
PROJECT_ROOT=`cd $SCRIPT_DIR/../../.. && pwd`

sed "s/%CACHE_VERSION%/`date +'%s'`/" $SCRIPT_DIR/service-worker.js.template > $SCRIPT_DIR/service-worker.js

rm -r $SCRIPT_DIR/assets
cp -a $PROJECT_ROOT/client/assets $SCRIPT_DIR/assets

echo "Compiling Game Engine..."
if [ $PROFILE == "debug" ]; then
    cargo build --target wasm32-unknown-unknown
else
    cargo build --target wasm32-unknown-unknown --release
fi

echo "Generating Usable Wasm Binary and Supporting Files..."
if [ $PROFILE == "debug" ]; then
    wasm-bindgen $PROJECT_ROOT/target/wasm32-unknown-unknown/debug/main.wasm --out-dir $SCRIPT_DIR/pkg --typescript --target web --debug --keep-debug
else
    wasm-bindgen $PROJECT_ROOT/target/wasm32-unknown-unknown/release/main.wasm --out-dir $SCRIPT_DIR/pkg --typescript --target web
fi

if [ $PROFILE != "debug" ]; then
    echo "Optimizing Wasm Binary For Size..."
    wasm-opt $SCRIPT_DIR/pkg/main_bg.wasm -o $SCRIPT_DIR/pkg/main_bg.opt.wasm -O
    mv $SCRIPT_DIR/pkg/main_bg.opt.wasm $SCRIPT_DIR/pkg/main_bg.wasm
fi

echo "Create Wasm Source Map..."
cargo wasm2map $SCRIPT_DIR/pkg/main_bg.wasm --patch --base-url $HOST
