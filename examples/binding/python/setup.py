#!/usr/bin/env python3

# Extensions - https://stackoverflow.com/a/18032741
# .py - Regular Python file
# .pyx - Cython file to be compiled to C/C++
# .pxd - Cython header file

import os
import sys

from setuptools import setup, Extension
from Cython.Build import cythonize


def get_script_path():
    return os.path.dirname(os.path.realpath(sys.argv[0]))

def get_project_root():
    return os.path.realpath(f"{get_script_path()}/../../..")

def get_include_dirs():
    return os.path.realpath(f"{get_project_root()}/target/binding")

def get_build_profile():
    return str(os.environ.get("RUSTUP_PROFILE", "debug"))

def get_library_dirs():
    return os.path.realpath(f"{get_project_root()}/target/{get_build_profile()}")

# print(get_library_dirs())

print(f"Building with {get_build_profile()} profile...")
print(f"Project Root: {get_project_root()}")
print(f"Include Directories: {get_include_dirs()}")
print(f"Library Directories: {get_library_dirs()}")

setup(
    name='Catgirl Engine Cython Demo',
    ext_modules=[
        Extension(
            name="catgirl_engine",
            sources=["catgirl_engine.pyx"],
            include_dirs=[get_include_dirs()],
            library_dirs=[get_library_dirs()],
            libraries=['main']
        ),
    ]
)
