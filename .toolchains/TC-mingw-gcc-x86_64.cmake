# Target Operating System
set(CMAKE_SYSTEM_NAME Windows)

# C and C++ Compilers as well as Static Linker (AR)
set(CMAKE_AR   x86_64-w64-mingw32-gcc-ar-posix)
set(CMAKE_C_COMPILER   x86_64-w64-mingw32-gcc-posix)
set(CMAKE_CXX_COMPILER x86_64-w64-mingw32-g++-posix)

# Set Root Paths
set(CMAKE_FIND_ROOT_PATH /usr/bin)

# NEVER Means Don't Use CMAKE_FIND_ROOT_PATH,
# Only Means Only Use CMAKE_FIND_ROOT_PATH
# Both Means Use Both Standard Paths and CMAKE_FIND_ROOT_PATH
set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)  # /lib
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)  # /include
set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM BOTH)  # /bin

# Enable Position Indepent Code (-fPIC)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)