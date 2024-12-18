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
if [ -z "$PIP_EXE" ]; then
    export PIP_EXE=`which pip`  # /usr/bin/pip
fi

if [ -z "$PRE_COMMIT_EXE" ]; then
    export PRE_COMMIT_EXE=`which pre-commit`  # /usr/bin/pre-commit
fi

echo "Install Python Pre-Commit Executable..."
$PIP_EXE install pre-commit

echo "Install Pre-Commit Hook..."
$PRE_COMMIT_EXE install
