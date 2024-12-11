#!/bin/bash
if [ -z "$RUSTUP_TOOLCHAIN" ]; then
    export RUSTUP_TOOLCHAIN="stable"  # "stable" or "nightly"
fi

if [ -z "$RUSTUP_PROFILE" ]; then
    export RUSTUP_PROFILE="debug"  # "debug" or "release"
fi

# Build Time Autovars
SCRIPT=`realpath "$0"`
SCRIPT_DIR=`dirname "$SCRIPT"`
PROJECT_ROOT=$SCRIPT_DIR

echo "Building docs..."
if [ $RUSTUP_PROFILE == "release" ]; then
    cargo +$RUSTUP_TOOLCHAIN doc --no-deps --workspace --all-features --document-private-items --release
else
    cargo +$RUSTUP_TOOLCHAIN doc --no-deps --workspace --all-features --document-private-items
fi
