#!/bin/sh

SCRIPT=`realpath "$0"`
SCRIPT_DIR=`dirname "$SCRIPT"`
PROJECT_ROOT=`realpath $SCRIPT_DIR/../../..`

cd $PROJECT_ROOT
mkdir -p $PROJECT_ROOT/target/examples

echo Building Catgirl-Engine...
cargo build --release --lib

echo Compiling C++ Program...
g++ $PROJECT_ROOT/examples/binding/cpp/main.cpp -I$PROJECT_ROOT/target/binding -L$PROJECT_ROOT/target/release -lmain -Wl,-rpath,.,-rpath,target/release -o $PROJECT_ROOT/target/examples/examplecpp.run
