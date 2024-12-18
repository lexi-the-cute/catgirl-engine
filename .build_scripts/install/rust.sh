#!/bin/bash
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
PROJECT_ROOT=`$REALPATH $SCRIPT_DIR/../..`

# Shell Command Locations
if [ -z "$MKDIR" ]; then
    export MKDIR=`which mkdir`  # /usr/bin/mkdir
fi

if [ -z "$CURL" ]; then
    export CURL=`which curl`  # /usr/bin/curl
fi

if [ -z "$CHMOD" ]; then
    export CHMOD=`which chmod`  # /usr/bin/chmod
fi

# Script Vars
if [ -z "$RUSTUP_TOOLCHAIN" ]; then
    export RUSTUP_TOOLCHAIN="stable"  # "stable" or "nightly"
fi

if [ -z "$RUSTUP_PROFILE" ]; then
    export RUSTUP_PROFILE="release"  # "debug" or "release"
fi

if [ -z "$RUST_INSTALLER_URL" ]; then
    export RUST_INSTALLER_URL="https://sh.rustup.rs"
fi

if [ -z "$RUSTUP_TARGETS" ]; then
    export RUSTUP_TARGETS="x86_64-unknown-linux-gnu x86_64-unknown-linux-musl x86_64-pc-windows-gnu wasm32-unknown-unknown armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android"  # "x86_64-unknown-linux-gnu"
fi

if [ -z "$ROOT_PATH" ]; then
    if [ "$WORKSPACE" ]; then
        # If workspace is specified like on CI, then stick on home directory
        export ROOT_PATH=$HOME/.tools
    else
        # Else, keep all tools local
        export ROOT_PATH=$PROJECT_ROOT/.tools
    fi
fi

if [ -z "$TOOLS_PATH" ]; then
    export TOOLS_PATH=$ROOT_PATH/.tools
fi

echo "Project Root: $PROJECT_ROOT"
echo "Toolchain: $RUSTUP_TOOLCHAIN - Build Profile: $RUSTUP_PROFILE"
echo "Targets: $RUSTUP_TARGETS"

echo "Creating Tools Directory..."
$MKDIR -p "$TOOLS_PATH"

echo "Downloading Rust Installer..."
$CURL --proto '=https' --tlsv1.2 --silent --show-error --fail --location $RUST_INSTALLER_URL > "$TOOLS_PATH/rust.sh"

echo "Marking Rust Installer as Executable..."
$CHMOD +x "$TOOLS_PATH/rust.sh"

echo "Installing Rust..."
$TOOLS_PATH/rust.sh -y

echo "Load Cargo Environment Variables..."
source "$HOME/.cargo/env"

if [ -z "$RUSTUP" ]; then
    export RUSTUP=`which rustup`  # ~/.cargo/bin/rustup
fi

echo "Installing $RUSTUP_TOOLCHAIN toolchain as default..."
$RUSTUP default $RUSTUP_TOOLCHAIN

echo "Install Rust targets..."
$RUSTUP target add --toolchain $RUSTUP_TOOLCHAIN $RUSTUP_TARGETS
