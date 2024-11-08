#!/bin/sh
HOST=http://127.0.0.1:8000/pkg
# RUST_LOG=info

# Build Time Autovars
SCRIPT=`realpath "$0"`
SCRIPT_DIR=`dirname "$SCRIPT"`
PROJECT_ROOT=`cd $SCRIPT_DIR/../../.. && pwd`

sed "s/%CACHE_VERSION%/`date +'%s'`/" $SCRIPT_DIR/service-worker.js.template > $SCRIPT_DIR/service-worker.js

rm -r $SCRIPT_DIR/assets
cp -a $PROJECT_ROOT/License.md $PROJECT_ROOT/LICENSE
cp -a $PROJECT_ROOT/client/assets $SCRIPT_DIR/assets

# Generates usable Wasm binary and supporting files
# wasm-pack build --profiling --target web -d $SCRIPT_DIR/pkg
wasm-pack build --dev --target web -d $SCRIPT_DIR/pkg
# wasm-bindgen $PROJECT_ROOT/target/wasm32-unknown-unknown/debug/main.wasm --out-dir $SCRIPT_DIR/pkg --typescript --target web --debug

# Creates Wasm Source Map to aid in debugging
cargo wasm2map $SCRIPT_DIR/pkg/main_bg.wasm --patch --base-url $HOST

# cargo build --target wasm32-unknown-unknown
# wasm-bindgen --keep-debug --web --out-dir pkg ./target/wasm32-unknown-unknown/

rm $PROJECT_ROOT/LICENSE
