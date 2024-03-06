#!/bin/sh

# printf \\33\[\?1047h
tput smcup
clear
echo "This currently doesn't work..."
echo "Press enter to run anyway..."
echo "For more info, see https://github.com/lexi-the-cute/catgirl-engine/issues/2"
read PAUSE

mkdir -p ../../../target/examples
pip3 install Cython

cargo build --release --lib
cp -a target/binding/catgirl_engine.pxd .
tput rmcup
# printf \\33\[\?1047l

python3 main.py
