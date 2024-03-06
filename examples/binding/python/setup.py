#!/usr/bin/env python3

# Extensions - https://stackoverflow.com/a/18032741
# .py - Regular Python file
# .pyx - Cython file to be compiled to C/C++
# .pxd - Cython header file

from setuptools import setup
from Cython.Build import cythonize

setup(
    name='Catgirl Engine Cython Demo',
    ext_modules=cythonize("catgirl_engine.pyx")
)
