import sys
sys.path.insert(0, 'target/binding')  # This runs from CWD
sys.path.insert(1, 'target/release')

import pyximport
pyximport.install()

import catgirl_engine

if __name__ == "__main__":
    args: list = []

    print(dir(catgirl_engine))
    print(catgirl_engine.__loader__) #.SDL_main(len(args), args)