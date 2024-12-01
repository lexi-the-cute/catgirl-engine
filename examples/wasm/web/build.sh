#!/bin/sh
TOOLCHAIN="stable"  # "stable" or "nightly"
PROFILE="debug"  # "debug" or "release"
ENABLE_SOURCES="true"
HOST=http://127.0.0.1:8000/pkg
# RUST_LOG=info

# Build Time Vars
SCRIPT=`realpath "$0"`
SCRIPT_DIR=`dirname "$SCRIPT"`
PROJECT_ROOT=`cd $SCRIPT_DIR/../../.. && pwd`
LIBRARY_PATH="$HOME/.rustup/toolchains/$TOOLCHAIN-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library"

sed "s/%CACHE_VERSION%/`date +'%s'`/" $SCRIPT_DIR/service-worker.js.template > $SCRIPT_DIR/service-worker.js

rm -r $SCRIPT_DIR/pkg
rm -r $SCRIPT_DIR/assets
cp -a $PROJECT_ROOT/client/assets $SCRIPT_DIR/assets

echo "Compiling Game Engine..."
if [ $PROFILE == "debug" ]; then
    cargo +$TOOLCHAIN build --target wasm32-unknown-unknown --bin catgirl-engine
else
    cargo +$TOOLCHAIN build --target wasm32-unknown-unknown --release --bin catgirl-engine
fi

echo "Generating Usable Wasm Binary and Supporting Files..."
if [ $PROFILE == "debug" ]; then
    wasm-bindgen $PROJECT_ROOT/target/wasm32-unknown-unknown/debug/catgirl-engine.wasm --out-dir $SCRIPT_DIR/pkg --typescript --target web --debug --keep-debug
else
    wasm-bindgen $PROJECT_ROOT/target/wasm32-unknown-unknown/release/catgirl-engine.wasm --out-dir $SCRIPT_DIR/pkg --typescript --target web
fi

if [ $PROFILE != "debug" ]; then
    echo "Optimizing Wasm Binary For Size..."
    wasm-opt $SCRIPT_DIR/pkg/catgirl-engine_bg.wasm -o $SCRIPT_DIR/pkg/catgirl-engine_bg.opt.wasm -Oz
    mv $SCRIPT_DIR/pkg/catgirl-engine_bg.opt.wasm $SCRIPT_DIR/pkg/catgirl-engine_bg.wasm
fi

if [ $PROFILE == "debug" ] && [ $ENABLE_SOURCES == "true" ]; then
    echo "Create Wasm Source Map..."
    cargo wasm2map $SCRIPT_DIR/pkg/catgirl-engine_bg.wasm --patch --base-url $HOST
fi

# if [ $PROFILE == "debug" ] && [ $ENABLE_SOURCES == "true" ]; then
#     echo "Symlinking Rust Sources..."
#     ln -s $LIBRARY_PATH/* $SCRIPT_DIR/pkg/
# fi
