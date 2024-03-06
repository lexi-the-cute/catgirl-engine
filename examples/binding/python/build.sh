#!/bin/sh

# printf \\33\[\?1047h
tput smcup
clear
echo Cleaning Old Build
chmod +x ./clean.sh
./clean.sh

echo Installing Cython
pip3 install Cython

echo Building Catgirl-Engine
cargo build --release --lib

echo Copying Cython Header
cp -a ../../../target/binding/catgirl_engine.pxd .

echo Running setup.py
./setup.py build_ext --inplace
tput rmcup
# printf \\33\[\?1047l

# Needed for main.py to find libmain.so
export LD_LIBRARY_PATH=`realpath ../../../target/release`
./main.py
