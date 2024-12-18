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

cd $PROJECT_ROOT

# Shell Command Locations
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

echo "Activating Python Virtual Environment..."
source $TOOLS_PATH/.venv/bin/activate

if [ -z "$PRE_COMMIT_EXE" ]; then
    export PRE_COMMIT_EXE=`which pre-commit`  # /usr/bin/pre-commit
fi

echo "Run Pre-Commit Hook..."
$PRE_COMMIT_EXE run --all-files
