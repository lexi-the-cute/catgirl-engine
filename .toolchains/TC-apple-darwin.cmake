# Target Operating System
set(CMAKE_SYSTEM_NAME Linux)

# C and C++ Compilers as well as Static Linker (AR)
set(CMAKE_AR   ar)  # /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/ar
set(CMAKE_C_COMPILER   /tmp/build-scripts/clang-fat.sh)
set(CMAKE_CXX_COMPILER /tmp/build-scripts/clang++-fat.sh)

# Set Root Paths
set(CMAKE_FIND_ROOT_PATH /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin /usr/local/include)

# NEVER Means Don't Use CMAKE_FIND_ROOT_PATH,
# Only Means Only Use CMAKE_FIND_ROOT_PATH
# Both Means Use Both Standard Paths and CMAKE_FIND_ROOT_PATH
set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)  # /lib
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)  # /include
set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM BOTH)  # /bin

# Enable Position Indepent Code (-fPIC)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)