#!/bin/env bash
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

cd $PROJECT_ROOT

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

if [ -z "$RUSTUP_TARGETS" ]; then
    export RUSTUP_TARGETS="x86_64-unknown-linux-gnu"  # "x86_64-unknown-linux-gnu"
fi

if [ -z "$REINSTALL_TOOLS" ]; then
    export REINSTALL_TOOLS="false"  # "true" or "false"
fi

FORCE_FLAG=""
if [ $REINSTALL_TOOLS == "true" ]; then
    FORCE_FLAG="--force"
fi

if [ -z "$BUILD_PLATFORM" ]; then
    export BUILD_PLATFORM="x86_64"  # "x86_64" or "i686" or "armhf" or "aarch64"
fi

if [ -z "$APPIMAGE_TOOL_URL" ]; then
    export APPIMAGE_TOOL_URL="https://github.com/foxgirl-labs/appimagetool/releases/download/continuous/appimagetool-$BUILD_PLATFORM.AppImage"
fi

if [ -z "$APPIMAGE_RUNTIME_URL" ]; then
    if [ $RUSTUP_PROFILE == "release" ]; then
        export APPIMAGE_RUNTIME_URL="https://github.com/foxgirl-labs/appimagetool/releases/download/continuous/appimagetool-$BUILD_PLATFORM.AppImage"
    else
        export APPIMAGE_RUNTIME_URL="https://github.com/foxgirl-labs/appimagetool/releases/download/continuous/appimagetool-$BUILD_PLATFORM.AppImage.debug"
    fi
fi

if [ -z "$CARGO_APPIMAGE_URL" ]; then
    export CARGO_APPIMAGE_URL="https://github.com/foxgirl-labs/cargo-appimage"
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

"$SCRIPT_DIR/rust.sh"

if [ -z "$CARGO_EXE" ]; then
    export CARGO_EXE=`which cargo`  # ~/.cargo/bin/cargo
fi

if [ ! -f "$TOOLS_PATH/appimagetool" ] || [ $REINSTALL_TOOLS == "true" ]; then
    echo "Creating Tools Directory..."
    $MKDIR_EXE -p "$TOOLS_PATH"

    echo "Install Customized AppImage Tool..."
    $CURL_EXE --proto '=https' --tlsv1.2 --silent --show-error --fail --location "$APPIMAGE_TOOL_URL" > $TOOLS_PATH/appimagetool

    CURL_EXIT_CODE=$?
    if [ $CURL_EXIT_CODE -ne 0 ]; then
        echo "Curl command failed with exit code $CURL_EXIT_CODE..."
        exit $CURL_EXIT_CODE
    fi

    echo "Marking AppImage Tool as Executable..."
    $CHMOD_EXE +x $TOOLS_PATH/appimagetool

    echo "Install Customized Cargo AppImage Tool..."
    if [ $RUSTUP_PROFILE == "release" ]; then
        $CARGO_EXE +$RUSTUP_TOOLCHAIN install --git "$CARGO_APPIMAGE_URL" $FORCE_FLAG
    else
        $CARGO_EXE +$RUSTUP_TOOLCHAIN install --git "$CARGO_APPIMAGE_URL" --debug $FORCE_FLAG
    fi

    echo "Install AppImage Runtime..."
    $CURL_EXE --proto '=https' --tlsv1.2 --silent --show-error --fail --location "$APPIMAGE_RUNTIME_URL" > $TOOLS_PATH/runtime-$BUILD_PLATFORM

    CURL_EXIT_CODE=$?
    if [ $CURL_EXIT_CODE -ne 0 ]; then
        echo "Curl command failed with exit code $CURL_EXIT_CODE..."
        exit $CURL_EXIT_CODE
    fi

    echo "Marking AppImage Runtime as Executable..."
    $CHMOD_EXE +x $TOOLS_PATH/runtime-$BUILD_PLATFORM
fi
