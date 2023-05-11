# Target Operating System
set(CMAKE_SYSTEM_NAME Linux)

# C and C++ Compilers as well as Static Linker (AR)
set(CMAKE_AR   aarch64-linux-gnu-gcc-ar)
set(CMAKE_C_COMPILER   aarch64-linux-gnu-gcc)
set(CMAKE_CXX_COMPILER aarch64-linux-gnu-g++)

# Set Root Paths
set(CMAKE_FIND_ROOT_PATH /usr/bin /usr/lib/aarch64-linux-gnu /usr/include/aarch64-linux-gnu)

# NEVER Means Don't Use CMAKE_FIND_ROOT_PATH,
# Only Means Only Use CMAKE_FIND_ROOT_PATH
# Both Means Use Both Standard Paths and CMAKE_FIND_ROOT_PATH
set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)  # /lib
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)  # /include
set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM BOTH)  # /bin

# Enable Position Indepent Code (-fPIC)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)