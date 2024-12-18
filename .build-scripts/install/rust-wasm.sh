#!/bin/bash
# Setup Bash Safety Checks
set -eo pipefail

# Setup for Build Time Autovars
if [ -z "$REALPATH_EXE" ]; then
    export REALPATH_EXE=`which realpath`  # /usr/bin/realpath
fi

if [ -z "$DIRNAME_EXE" ]; then
    export DIRNAME_EXE=`which dirname`  # /usr/bin/dirname
fi

# Build Time Autovars
SCRIPT=`$REALPATH_EXE "$0"`
SCRIPT_DIR=`$DIRNAME_EXE "$SCRIPT"`
PROJECT_ROOT=`$REALPATH_EXE $SCRIPT_DIR/../..`

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
fi

FORCE_FLAG=""
if [ $REINSTALL_TOOLS == "true" ]; then
    FORCE_FLAG="--force"
fi

echo "Installing Rust..."
"$SCRIPT_DIR/rust.sh"

if [ -z "$CARGO_EXE" ]; then
    export CARGO_EXE=`which cargo`  # ~/.cargo/bin/cargo
fi

echo "Install Wasm-Bindgen Tools..."
if [ $RUSTUP_PROFILE == "release" ]; then
    $CARGO_EXE +$RUSTUP_TOOLCHAIN install wasm-bindgen-cli --version $WASM_BINDGEN_VERSION $FORCE_FLAG
else
    $CARGO_EXE +$RUSTUP_TOOLCHAIN install wasm-bindgen-cli --version $WASM_BINDGEN_VERSION --debug $FORCE_FLAG
fi

echo "Install Wasm Optimization Tools..."
if [ $RUSTUP_PROFILE == "release" ]; then
    $CARGO_EXE +$RUSTUP_TOOLCHAIN install wasm-opt $FORCE_FLAG
else
    $CARGO_EXE +$RUSTUP_TOOLCHAIN install wasm-opt --debug $FORCE_FLAG
fi

echo "Install Wasm Source Mapping Tools..."
if [ $RUSTUP_PROFILE == "release" ]; then
    $CARGO_EXE +$RUSTUP_TOOLCHAIN install cargo-wasm2map $FORCE_FLAG
else
    $CARGO_EXE +$RUSTUP_TOOLCHAIN install cargo-wasm2map --debug $FORCE_FLAG
fi
