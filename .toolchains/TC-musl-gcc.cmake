# Target Operating System
set(CMAKE_SYSTEM_NAME Linux)

# C and C++ Compilers
set(CMAKE_C_COMPILER   x86_64-linux-musl-gcc)
set(CMAKE_CXX_COMPILER x86_64-linux-musl-g++)

# Set System Paths
set(CMAKE_SYSTEM_INCLUDE_PATH /usr/x86_64-linux-gnu/include $ENV{WORKSPACE}/build/x86-64-linux-musl-cross/x86_64-linux-musl/include $ENV{WORKSPACE}/build/x86-64-linux-musl-cross/include)  # /include
set(CMAKE_SYSTEM_LIBRARY_PATH $ENV{WORKSPACE}/build/x86-64-linux-musl-cross/x86_64-linux-musl/lib $ENV{WORKSPACE}/build/x86-64-linux-musl-cross/lib)  # /lib
set(CMAKE_SYSTEM_PROGRAM_PATH $ENV{WORKSPACE}/build/x86-64-linux-musl-cross/x86_64-linux-musl/bin $ENV{WORKSPACE}/build/x86-64-linux-musl-cross/bin)  # /bin