#!/bin/sh

SCRIPT=`realpath "$0"`
SCRIPT_DIR=`dirname "$SCRIPT"`
PROJECT_ROOT=`realpath $SCRIPT_DIR/../../..`

cd $PROJECT_ROOT

echo Building Catgirl-Engine...
cargo build --release --lib

echo Copying Cython Header...
cp -a $PROJECT_ROOT/target/binding/catgirl_engine.pxd $SCRIPT_DIR

echo Running setup.py...
cd $SCRIPT_DIR && $SCRIPT_DIR/setup.py build_ext --inplace
