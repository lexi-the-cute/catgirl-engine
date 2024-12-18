#!/bin/bash
# Setup Bash Safety Checks
set -eo pipefail

# Setup for Build Time Autovars
if [ -z "$REALPATH" ]; then
    export REALPATH=`which realpath`  # /usr/bin/realpath
fi

if [ -z "$DIRNAME" ]; then
    export DIRNAME=`which dirname`  # /usr/bin/dirname
fi

# Build Time Autovars
SCRIPT=`$REALPATH "$0"`
SCRIPT_DIR=`$DIRNAME "$SCRIPT"`
PROJECT_ROOT=`$REALPATH $SCRIPT_DIR/../../..`

# Script Vars
WASM_BINDGEN_VERSION=`cat $PROJECT_ROOT/Cargo.toml | grep '^wasm-bindgen' | head -n1 | cut -d'"' -f2 | tr -d '\n'`

if [ -z "$RUSTUP_TOOLCHAIN" ]; then
    export RUSTUP_TOOLCHAIN="stable"  # "stable" or "nightly"
fi

if [ -z "$RUSTUP_PROFILE" ]; then
    export RUSTUP_PROFILE="release"  # "debug" or "release"
fi

if [ -z "$RUSTUP_TARGETS" ]; then
    export RUSTUP_TARGETS="wasm32-unknown-unknown"  # "x86_64-unknown-linux-gnu"
fi

if [ -z "$REINSTALL_TOOLS" ]; then
    export REINSTALL_TOOLS="false"  # "true" or "false"
    FORCE_FLAG="--force"
fi

echo "Installing Rust..."
"$SCRIPT_DIR/rust.sh"

if [ -z "$CARGO" ]; then
    export CARGO=`which cargo`  # ~/.cargo/bin/cargo
fi

echo "Install Wasm-Bindgen Tools..."
if [ $RUSTUP_PROFILE == "release" ]; then
    $CARGO +$RUSTUP_TOOLCHAIN install wasm-bindgen-cli --version $WASM_BINDGEN_VERSION $FORCE_FLAG
else
    $CARGO +$RUSTUP_TOOLCHAIN install wasm-bindgen-cli --version $WASM_BINDGEN_VERSION --debug $FORCE_FLAG
fi

echo "Install Wasm Optimization Tools..."
if [ $RUSTUP_PROFILE == "release" ]; then
    $CARGO +$RUSTUP_TOOLCHAIN install wasm-opt $FORCE_FLAG
else
    $CARGO +$RUSTUP_TOOLCHAIN install wasm-opt --debug $FORCE_FLAG
fi

echo "Install Wasm Source Mapping Tools..."
if [ $RUSTUP_PROFILE == "release" ]; then
    $CARGO +$RUSTUP_TOOLCHAIN install cargo-wasm2map $FORCE_FLAG
else
    $CARGO +$RUSTUP_TOOLCHAIN install cargo-wasm2map --debug $FORCE_FLAG
fi
