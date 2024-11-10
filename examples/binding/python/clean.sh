#!/bin/sh

SCRIPT=`realpath "$0"`
SCRIPT_DIR=`dirname "$SCRIPT"`
PROJECT_ROOT=`realpath $SCRIPT_DIR/../../..`

cd $SCRIPT_DIR

rm *.c
rm *.pxd
rm *.so
rm -r $SCRIPT_DIR/build
