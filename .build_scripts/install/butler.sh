#!/bin/bash
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

if [ -z "$UNZIP" ]; then
    export UNZIP=`which unzip`  # /usr/bin/unzip
fi

# Script Vars
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

if [ -z "$BUTLER_URL" ]; then
    export BUTLER_URL="https://broth.itch.zone/butler/linux-amd64/LATEST/archive/default"
fi

# Build Time Autovars
SCRIPT=`$REALPATH "$0"`
SCRIPT_DIR=`$DIRNAME "$SCRIPT"`
PROJECT_ROOT=`$REALPATH $SCRIPT_DIR/../..`

echo "Creating Tools Directory..."
$MKDIR -p "$TOOLS_PATH"

echo "Download Itch.io Butler"
$CURL --proto '=https' --tlsv1.2 --silent --show-error --fail --location "$BUTLER_URL" > "$TOOLS_PATH/butler-linux-amd64.zip"

echo "Unzip Itch.io Butler"
$UNZIP -o "$TOOLS_PATH/butler-linux-amd64.zip" -d "$TOOLS_PATH/butler"
