# cython: language_level=3
# cython: libraries=main

from libc.stdlib cimport malloc, free

cimport catgirl_engine


def run(args: list):
    cdef const char** c_args
    c_args = <const char**> malloc(sizeof(const char*)*len(args))

    try:
        for n, a in enumerate(args):
            c_args[n] = a

        return catgirl_engine.ce_start(len(args), c_args)
    finally:
        free(c_args)

def run():
    #args = [b"--assets", b"path/to/assets/folder"]
    return run([])
