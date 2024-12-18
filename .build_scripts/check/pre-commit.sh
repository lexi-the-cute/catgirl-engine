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
if [ -z "$PIP" ]; then
    export PIP=`which pip`  # /usr/bin/pip
fi

if [ -z "$PRE_COMMIT" ]; then
    export PRE_COMMIT=`which pre-commit`  # /usr/bin/pre-commit
fi

# Build Time Autovars
SCRIPT=`$REALPATH "$0"`
SCRIPT_DIR=`$DIRNAME "$SCRIPT"`
PROJECT_ROOT=`$REALPATH $SCRIPT_DIR/../..`

echo "Run Pre-Commit Hook..."
$PRE_COMMIT run --all-files
