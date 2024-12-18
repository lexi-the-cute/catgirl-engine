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

# Shell Command Locations
if [ -z "$MKDIR_EXE" ]; then
    export MKDIR_EXE=`which mkdir`  # /usr/bin/mkdir
fi

if [ -z "$CURL_EXE" ]; then
    export CURL_EXE=`which curl`  # /usr/bin/curl
fi

if [ -z "$CHMOD_EXE" ]; then
    export CHMOD_EXE=`which chmod`  # /usr/bin/chmod
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

if [ -z "$REINSTALL_TOOLS" ]; then
    export REINSTALL_TOOLS="false"  # "true" or "false"
fi

if [ -z "$ROOT_PATH" ]; then
    if [ "$WORKSPACE" ]; then
        # If workspace is specified like on CI, then stick on home directory
        export ROOT_PATH=$HOME
    else
        # Else, keep all tools local
        export ROOT_PATH=$PROJECT_ROOT
    fi
fi

if [ -z "$TOOLS_PATH" ]; then
    export TOOLS_PATH=$ROOT_PATH/.tools
fi

echo "Project Root: $PROJECT_ROOT"
echo "Toolchain: $RUSTUP_TOOLCHAIN - Build Profile: $RUSTUP_PROFILE"
echo "Targets: $RUSTUP_TARGETS"

if [ ! -f "$TOOLS_PATH/rust.sh" ] || [ $REINSTALL_TOOLS == "true" ]; then
    echo "Creating Tools Directory..."
    $MKDIR_EXE -p "$TOOLS_PATH"

    echo "Downloading Rust Installer..."
    $CURL_EXE --proto '=https' --tlsv1.2 --silent --show-error --fail --location $RUST_INSTALLER_URL > "$TOOLS_PATH/rust.sh"

    CURL_EXIT_CODE=$?
    if [ $CURL_EXIT_CODE -ne 0 ]; then
        echo "Curl command failed with exit code $CURL_EXIT_CODE..."
        exit $CURL_EXIT_CODE
    fi

    echo "Marking Rust Installer as Executable..."
    $CHMOD_EXE +x "$TOOLS_PATH/rust.sh"

    echo "Installing Rust..."
    $TOOLS_PATH/rust.sh -y
fi

echo "Load Cargo Environment Variables..."
source "$HOME/.cargo/env"

if [ -z "$RUSTUP_EXE" ]; then
    export RUSTUP_EXE=`which rustup`  # ~/.cargo/bin/rustup
fi

echo "Installing $RUSTUP_TOOLCHAIN toolchain as default..."
$RUSTUP_EXE default $RUSTUP_TOOLCHAIN

echo "Install Rust targets..."
$RUSTUP_EXE target add --toolchain $RUSTUP_TOOLCHAIN $RUSTUP_TARGETS
