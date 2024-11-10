#!/bin/sh

SCRIPT=`realpath "$0"`
SCRIPT_DIR=`dirname "$SCRIPT"`
PROJECT_ROOT=`realpath $SCRIPT_DIR/../../..`

cd $PROJECT_ROOT
$PROJECT_ROOT/target/examples/examplec.run
