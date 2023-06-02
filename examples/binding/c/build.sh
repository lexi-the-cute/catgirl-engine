#!/bin/sh

cd ../../..
mkdir -p target/examples

cargo build --release --lib
gcc examples/binding/c/main.c -Itarget/binding -Ltarget/release -lmain -Wl,-rpath,.,-rpath,target/release -o target/examples/examplec.run
target/examples/examplec.run