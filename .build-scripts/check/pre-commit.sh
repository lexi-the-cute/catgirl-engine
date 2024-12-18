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

# Shell Command Locations
if [ -z "$PRE_COMMIT_EXE" ]; then
    export PRE_COMMIT_EXE=`which pre-commit`  # /usr/bin/pre-commit
fi

# Build Time Autovars
SCRIPT=`$REALPATH_EXE "$0"`
SCRIPT_DIR=`$DIRNAME_EXE "$SCRIPT"`
PROJECT_ROOT=`$REALPATH_EXE $SCRIPT_DIR/../..`

cd $PROJECT_ROOT

echo "Run Pre-Commit Hook..."
$PRE_COMMIT_EXE run --all-files
