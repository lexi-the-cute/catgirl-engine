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

if [ -z "$UNZIP_EXE" ]; then
    export UNZIP_EXE=`which unzip`  # /usr/bin/unzip
fi

# Script Vars
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

if [ -z "$BUTLER_URL" ]; then
    export BUTLER_URL="https://broth.itch.zone/butler/linux-amd64/LATEST/archive/default"
fi

if [ ! -f "$TOOLS_PATH/butler-linux-amd64.zip" ] || [ $REINSTALL_TOOLS == "true" ]; then
    echo "Creating Tools Directory..."
    $MKDIR_EXE -p "$TOOLS_PATH"

    echo "Download Itch.io Butler"
    $CURL_EXE --proto '=https' --tlsv1.2 --silent --show-error --fail --location "$BUTLER_URL" > "$TOOLS_PATH/butler-linux-amd64.zip"

    CURL_EXIT_CODE=$?
    if [ $CURL_EXIT_CODE -ne 0 ]; then
        echo "Curl command failed with exit code $CURL_EXIT_CODE..."
        exit $CURL_EXIT_CODE
    fi

    echo "Unzip Itch.io Butler"
    $UNZIP_EXE -o "$TOOLS_PATH/butler-linux-amd64.zip" -d "$TOOLS_PATH/butler"
fi
