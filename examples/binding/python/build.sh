#!/bin/sh
echo Building Catgirl-Engine
cargo build --release --lib

echo Copying Cython Header
cp -a ../../../target/binding/catgirl_engine.pxd .

echo Running setup.py
./setup.py build_ext --inplace
