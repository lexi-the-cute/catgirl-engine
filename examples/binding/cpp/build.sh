#!/bin/sh

cd ../../..
mkdir -p target/examples

cargo build --release --lib
g++ examples/binding/cpp/main.cpp -Itarget/binding -Ltarget/release -lmain -Wl,-rpath,.,-rpath,target/release -o target/examples/examplecpp.run
target/examples/examplecpp.run
