#!/bin/sh
# RUST_LOG=info
CURRENT_DIR=`pwd`
cd ../../..

cp -a `pwd`/License.md `pwd`/LICENSE

wasm-pack build --profiling --target web -d $CURRENT_DIR/pkg
# cargo build --target wasm32-unknown-unknown
# wasm-bindgen --keep-debug --web --out-dir pkg ./target/wasm32-unknown-unknown/

rm `pwd`/LICENSE
