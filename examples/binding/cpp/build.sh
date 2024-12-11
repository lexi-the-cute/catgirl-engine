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
mkdir -p $PROJECT_ROOT/target/examples

echo Building Catgirl-Engine...
if [ $RUSTUP_PROFILE == "release" ]; then
    cargo build --release --lib
else
    cargo build --lib
fi

echo Compiling C++ Program...
g++ $PROJECT_ROOT/examples/binding/cpp/main.cpp -I$PROJECT_ROOT/target/binding -L$PROJECT_ROOT/target/$RUSTUP_PROFILE -lmain -Wl,-rpath,.,-rpath,target/$RUSTUP_PROFILE -o $PROJECT_ROOT/target/examples/examplecpp.run
