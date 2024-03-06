#!/usr/bin/env python3

# Import catgirl_engine.pyx
import catgirl_engine


if __name__ == "__main__":
    args: list = ["--version"]

    catgirl_engine.run(args)
