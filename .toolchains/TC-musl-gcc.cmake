# Target Operating System
set(CMAKE_SYSTEM_NAME Linux)

# C and C++ Compilers
set(CMAKE_C_COMPILER   x86_64-linux-musl-gcc)
set(CMAKE_CXX_COMPILER x86_64-linux-musl-g++)

# Set Root Paths
set(CMAKE_FIND_ROOT_PATH $ENV{WORKSPACE}/build/x86-64-linux-musl-cross $ENV{WORKSPACE}/build/x86-64-linux-musl-cross/x86_64-linux-musl /usr/x86_64-linux-gnu)

# Set System Paths
# set(CMAKE_SYSTEM_INCLUDE_PATH /usr/x86_64-linux-gnu/include)  # /include
# set(CMAKE_SYSTEM_LIBRARY_PATH /lib)  # /lib
# set(CMAKE_SYSTEM_PROGRAM_PATH /bin)  # /bin

# NEVER Means Don't Use CMAKE_FIND_ROOT_PATH,
# Only Means Only Use CMAKE_FIND_ROOT_PATH
# Both Means Use Both Standard Paths and CMAKE_FIND_ROOT_PATH
set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY BOTH)  # /lib
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)  # /include
set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM BOTH)  # /bin