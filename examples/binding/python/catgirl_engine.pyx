# cython: language_level=3, c_string_encoding=utf8

from libc.stdlib cimport malloc, free

# Import catgirl_engine.pxd
cimport catgirl_engine


def run(args: list):
    cdef const char** c_args
    c_args = <const char**> malloc(sizeof(const char*)*len(args))

    try:
        for n, a in enumerate(args):
            c_args[n] = a

        return catgirl_engine.start_engine(len(args), c_args)
    finally:
        free(c_args)
