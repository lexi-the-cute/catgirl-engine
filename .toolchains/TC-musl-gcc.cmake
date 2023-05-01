# the name of the target operating system
set(CMAKE_SYSTEM_NAME Linux)

# which compilers to use for C and C++
set(CMAKE_C_COMPILER   x86_64-linux-musl-gcc)
set(CMAKE_CXX_COMPILER x86_64-linux-musl-g++)

# where is the target environment located
# TODO: Move From /usr/x86_64-linux-gnu/include to /usr/x86_64-linux-musl/include
set(CMAKE_FIND_ROOT_PATH  $ENV{WORKSPACE}/build/x86-64-linux-musl-cross/x86_64-linux-musl/include;/usr/x86_64-linux-gnu/include;$ENV{WORKSPACE}/build/x86-64-linux-musl-cross)

# adjust the default behavior of the FIND_XXX() commands:
# search programs in the host environment
set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NONE)

# search headers and libraries in the target environment
set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE BOTH)

# Cross Compiler Stuff
# SET (CMAKE_C_COMPILER_WORKS 1)
# SET (CMAKE_CXX_COMPILER_WORKS 1)