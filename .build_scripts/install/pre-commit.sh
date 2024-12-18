#!/bin/bash
# Setup Bash Safety Checks
set -eo pipefail

# Setup for Build Time Autovars
if [ -z "$REALPATH" ]; then
    export REALPATH=`which realpath`  # /usr/bin/realpath
fi

if [ -z "$DIRNAME" ]; then
    export DIRNAME=`which dirname`  # /usr/bin/dirname
fi

# Shell Command Locations
if [ -z "$PRE_COMMIT" ]; then
    export PRE_COMMIT=`which pre-commit`  # /usr/bin/pre-commit
fi

# Build Time Autovars
SCRIPT=`$REALPATH "$0"`
SCRIPT_DIR=`$DIRNAME "$SCRIPT"`
PROJECT_ROOT=`$REALPATH $SCRIPT_DIR/../..`

echo "Install Python Pre-Commit Executable..."
$PIP install pre-commit

echo "Install Pre-Commit Hook..."
$PRE_COMMIT install
