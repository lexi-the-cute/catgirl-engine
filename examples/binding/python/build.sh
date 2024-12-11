#!/bin/sh
if [ -z "$RUSTUP_TOOLCHAIN" ]; then
    export RUSTUP_TOOLCHAIN="stable"  # "stable" or "nightly"
fi

if [ -z "$RUSTUP_PROFILE" ]; then
    export RUSTUP_PROFILE="debug"  # "debug" or "release"
fi

SCRIPT=`realpath "$0"`
SCRIPT_DIR=`dirname "$SCRIPT"`
PROJECT_ROOT=`realpath $SCRIPT_DIR/../../..`

cd $PROJECT_ROOT

echo Building Catgirl-Engine...
if [ $RUSTUP_PROFILE == "release" ]; then
    cargo build --release --lib
else
    cargo build --lib
fi

echo Copying Cython Header...
cp -a $PROJECT_ROOT/target/binding/catgirl_engine.pxd $SCRIPT_DIR

echo Running setup.py...
cd $SCRIPT_DIR && $SCRIPT_DIR/setup.py build_ext --inplace
