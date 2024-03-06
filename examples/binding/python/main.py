#!/usr/bin/env python3

import os

# Import catgirl_engine.pyx
import catgirl_engine


if __name__ == "__main__":
    args: list = [os.path.realpath(__file__), "--version"]

    catgirl_engine.run(args)
