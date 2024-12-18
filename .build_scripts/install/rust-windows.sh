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
if [ -z "$RUSTUP_TOOLCHAIN" ]; then
    export RUSTUP_TOOLCHAIN="stable"  # "stable" or "nightly"
fi

if [ -z "$RUSTUP_PROFILE" ]; then
    export RUSTUP_PROFILE="release"  # "debug" or "release"
fi

if [ -z "$RUSTUP_TARGETS" ]; then
    export RUSTUP_TARGETS="x86_64-pc-windows-gnu"  # "x86_64-unknown-linux-gnu"
fi

echo "Installing Rust..."
"$SCRIPT_DIR/rust.sh"
