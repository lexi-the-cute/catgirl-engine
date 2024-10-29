#!/bin/sh

# Needed for main.py to find libmain.so
export LD_LIBRARY_PATH=`realpath ../../../target/release`
./main.py
