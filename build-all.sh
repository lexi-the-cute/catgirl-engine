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

# Build Vars
SOURCE_DATE_EPOCH="`git --no-pager log -1 --format="%at"`"
PATH="${{ github.workspace }}/tools:$PATH"
PATH="${{ github.workspace }}/tools/butler:$PATH"

echo "Building x86_64-unknown-linux-gnu as AppImage..."
cargo +$RUSTUP_TOOLCHAIN appimage --features appimage --target=x86_64-unknown-linux-gnu --bin catgirl-engine

echo "Building x86_64-unknown-linux-musl Server Only..."
if [ $RUSTUP_PROFILE == "release" ]; then
    cargo +$RUSTUP_TOOLCHAIN build --target=x86_64-unknown-linux-musl --release --bin catgirl-engine --no-default-features --features server,logging-subscriber
else
    cargo +$RUSTUP_TOOLCHAIN build --target=x86_64-unknown-linux-musl --bin catgirl-engine --no-default-features --features server,logging-subscriber
fi

echo "Building x86_64-pc-windows-gnu..."
if [ $RUSTUP_PROFILE == "release" ]; then
    cargo build --target=x86_64-pc-windows-gnu --release --bin catgirl-engine
else
    cargo build --target=x86_64-pc-windows-gnu --bin catgirl-engine
fi

echo "Building wasm32-unknown-unknown..."
$PROJECT_ROOT/examples/wasm/web/build.sh

echo "Building Android..."
touch $PROJECT_ROOT/android/local.properties
if [ $RUSTUP_PROFILE == "release" ]; then
    $PROJECT_ROOT/android/gradlew assembleRelease
else
    $PROJECT_ROOT/android/gradlew assemble
fi
