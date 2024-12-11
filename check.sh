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

echo "Checking licenses..."
cargo +$RUSTUP_TOOLCHAIN deny --all-features check licenses

echo "Running Cargo Check..."
if [ $RUSTUP_PROFILE == "release" ]; then
    cargo +$RUSTUP_TOOLCHAIN check --workspace --all-targets --all-features --bins --tests --benches --examples --release
else
    cargo +$RUSTUP_TOOLCHAIN check --workspace --all-targets --all-features --bins --tests --benches --examples
fi

echo "Running Unit Tests..."
if [ $RUSTUP_PROFILE == "release" ]; then
    cargo +$RUSTUP_TOOLCHAIN test --workspace --all-targets --all-features --bins --tests --benches --examples --release
else
    cargo +$RUSTUP_TOOLCHAIN test --workspace --all-targets --all-features --bins --tests --benches --examples
fi

echo "Running Clippy Tests..."
if [ $RUSTUP_PROFILE == "release" ]; then
    cargo +$RUSTUP_TOOLCHAIN clippy --workspace --all-targets --all-features --release
else
    cargo +$RUSTUP_TOOLCHAIN clippy --workspace --all-targets --all-features
fi

echo "Running Future Compatibilities Reports..."
cargo +$RUSTUP_TOOLCHAIN report future-incompatibilities --package catgirl-engine || true  # `|| true` ignores the exit code
