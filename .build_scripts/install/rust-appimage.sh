# Setup for Build Time Autovars
if [ -z "$REALPATH" ]; then
    export REALPATH=`which realpath`  # /usr/bin/realpath
fi

if [ -z "$DIRNAME" ]; then
    export DIRNAME=`which dirname`  # /usr/bin/dirname
fi

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

# Build Time Autovars
SCRIPT=`$REALPATH "$0"`
SCRIPT_DIR=`$DIRNAME "$SCRIPT"`
PROJECT_ROOT=`$REALPATH $SCRIPT_DIR/../../..`

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
        export ROOT_PATH=$HOME/.tools
    else
        # Else, keep all tools local
        export ROOT_PATH=$PROJECT_ROOT/.tools
    fi
fi

if [ -z "$TOOLS_PATH" ]; then
    export TOOLS_PATH=$ROOT_PATH/.tools
fi

echo "Installing Rust..."
"$SCRIPT_DIR/rust.sh"

if [ -z "$CARGO" ]; then
    export CARGO=`which cargo`  # ~/.cargo/bin/cargo
fi

echo "Creating Tools Directory..."
$MKDIR -p "$TOOLS_PATH"

echo "Install Customized AppImage Tool..."
$CURL --proto '=https' --tlsv1.2 --silent --show-error --fail --location "$APPIMAGE_TOOL_URL" > $TOOLS_PATH/appimagetool

echo "Marking AppImage Tool as Executable..."
$CHMOD +x $TOOLS_PATH/appimagetool

echo "Install Customized Cargo AppImage Tool..."
if [ $RUSTUP_PROFILE == "release" ]; then
    $CARGO +$RUSTUP_TOOLCHAIN install --git "$CARGO_APPIMAGE_URL" $FORCE_FLAG
else
    $CARGO +$RUSTUP_TOOLCHAIN install --git "$CARGO_APPIMAGE_URL" --debug $FORCE_FLAG
fi

echo "Install AppImage Runtime..."
$CURL --proto '=https' --tlsv1.2 --silent --show-error --fail --location "$APPIMAGE_RUNTIME_URL" > $TOOLS_PATH/runtime-$BUILD_PLATFORM

echo "Marking AppImage Runtime as Executable..."
$CHMOD +x $TOOLS_PATH/runtime-$BUILD_PLATFORM
