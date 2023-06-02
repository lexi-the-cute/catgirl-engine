#!/bin/sh

echo "This currently doesn't work..."
echo "Press enter to run anyway..."
echo "For more info, see https://github.com/alexisart/catgirl-engine/issues/2"
read PAUSE

cd ../../..
mkdir -p target/examples
pip3 install Cython

cargo build --release --lib
python3 examples/binding/python/main.py