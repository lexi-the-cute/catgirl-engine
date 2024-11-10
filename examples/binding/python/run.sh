#!/bin/sh

SCRIPT=`realpath "$0"`
SCRIPT_DIR=`dirname "$SCRIPT"`
PROJECT_ROOT=`realpath $SCRIPT_DIR/../../..`

# Needed for main.py to find libmain.so
export LD_LIBRARY_PATH=`realpath $PROJECT_ROOT/target/release`
$SCRIPT_DIR/main.py
