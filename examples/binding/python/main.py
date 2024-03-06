#!/usr/bin/env python3

# Extensions - https://stackoverflow.com/a/18032741
# .py - Regular Python file
# .pyx - Cython file to be compiled to C/C++
# .pxd - Cython header file

# import sys
# sys.path.insert(0, '../../../target/release')

# Install Cython code handler
import pyximport
pyximport.install()

# Import catgirl_engine.pyx
import catgirl_engine


if __name__ == "__main__":
    args: list = []

    catgirl_engine.run(args)
