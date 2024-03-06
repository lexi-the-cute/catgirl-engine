#!/bin/sh

# printf \\33\[\?1047h
tput smcup
clear
echo Installing Cython
pip3 install Cython

echo Building Catgirl-Engine
cargo build --release --lib

echo Copying Cython Header
cp -a ../../../target/binding/catgirl_engine.pxd .

echo Running setup.py
python3 setup.py build_ext --inplace
tput rmcup
# printf \\33\[\?1047l

python3 main.py
