#!/bin/sh
if [ -z "$RUSTUP_TOOLCHAIN" ]; then
    export RUSTUP_TOOLCHAIN="stable"  # "stable" or "nightly"
fi

if [ -z "$RUSTUP_PROFILE" ]; then
    export RUSTUP_PROFILE="debug"  # "debug" or "release"
fi

ENABLE_SOURCES="true"
HOST=http://127.0.0.1:8000
PKG_URL=$HOST/pkg
# RUST_LOG=info

# Build Time Vars
SCRIPT=`realpath "$0"`
SCRIPT_DIR=`dirname "$SCRIPT"`
PROJECT_ROOT=`cd $SCRIPT_DIR/../../.. && pwd`
LIBRARY_PATH="$HOME/.rustup/RUSTUP_TOOLCHAINs/$RUSTUP_TOOLCHAIN-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library"

sed "s/%CACHE_VERSION%/`date +'%s'`/" $SCRIPT_DIR/service-worker.js.template > $SCRIPT_DIR/service-worker.js

rm -r $SCRIPT_DIR/pkg
rm -r $SCRIPT_DIR/resources

mkdir -p $SCRIPT_DIR/resources
cp -a $PROJECT_ROOT/resources/assets $SCRIPT_DIR/resources/assets

echo "Compiling Game Engine..."
if [ $RUSTUP_PROFILE == "debug" ]; then
    cargo +$RUSTUP_TOOLCHAIN build --target wasm32-unknown-unknown --lib
else
    cargo +$RUSTUP_TOOLCHAIN build --target wasm32-unknown-unknown --release --lib
fi

echo "Generating Usable Wasm Binary and Supporting Files..."
if [ $RUSTUP_PROFILE == "debug" ]; then
    wasm-bindgen $PROJECT_ROOT/target/wasm32-unknown-unknown/debug/main.wasm --out-dir $SCRIPT_DIR/pkg --typescript --target web --debug --keep-debug
else
    wasm-bindgen $PROJECT_ROOT/target/wasm32-unknown-unknown/release/main.wasm --out-dir $SCRIPT_DIR/pkg --typescript --target web
fi

if [ $RUSTUP_PROFILE != "debug" ]; then
    echo "Optimizing Wasm Binary For Size..."
    wasm-opt $SCRIPT_DIR/pkg/main_bg.wasm -o $SCRIPT_DIR/pkg/main_bg.opt.wasm -Oz
    mv $SCRIPT_DIR/pkg/main_bg.opt.wasm $SCRIPT_DIR/pkg/main_bg.wasm
fi

if [ $RUSTUP_PROFILE == "debug" ] && [ $ENABLE_SOURCES == "true" ]; then
    echo "Create Wasm Source Map..."
    cargo wasm2map $SCRIPT_DIR/pkg/main_bg.wasm --patch --base-url $PKG_URL
fi

# if [ $RUSTUP_PROFILE == "debug" ] && [ $ENABLE_SOURCES == "true" ]; then
#     echo "Symlinking Rust Sources..."
#     ln -s $LIBRARY_PATH/* $SCRIPT_DIR/pkg/
# fi
