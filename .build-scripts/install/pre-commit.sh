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
if [ -z "$MKDIR_EXE" ]; then
    export MKDIR_EXE=`which mkdir`  # /usr/bin/mkdir
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

if [ -z "$SYSTEM_PYTHON_EXE" ]; then
    export SYSTEM_PYTHON_EXE=`which python3`  # /usr/bin/pip3
fi

if [ ! -f "$TOOLS_PATH/.venv" ] || [ $REINSTALL_TOOLS == "true" ]; then
    echo "Creating Tools Directory..."
    $MKDIR_EXE -p "$TOOLS_PATH"

    echo "Creating Python Virtual Environment..."
    $SYSTEM_PYTHON_EXE -m venv $TOOLS_PATH/.venv
fi

echo "Activating Python Virtual Environment..."
source $TOOLS_PATH/.venv/bin/activate

if [ -z "$PIP_EXE" ]; then
    export PIP_EXE=`which pip3`  # /usr/bin/pip3
fi

echo "Updating Python Pip..."
$PIP_EXE install --upgrade pip

echo "Install Python Pre-Commit Executable..."
$PIP_EXE install --upgrade pre-commit

if [ -z "$PRE_COMMIT_EXE" ]; then
    export PRE_COMMIT_EXE=`which pre-commit`  # /usr/bin/pre-commit
fi

echo "Install Pre-Commit Hook..."
$PRE_COMMIT_EXE install
