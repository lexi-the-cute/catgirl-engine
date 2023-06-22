#!/usr/bin/env python3

# import sys
# sys.path.insert(0, 'target/binding')  # This runs from CWD
# sys.path.insert(1, 'target/release')

import pyximport
pyximport.install()
# LibLoader

import catgirl_engine

if __name__ == "__main__":
    args: list = []

    catgirl_engine.run(args)