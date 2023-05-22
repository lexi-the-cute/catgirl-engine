cmake_minimum_required(VERSION 3.6)

project(GAME)

# Debug Information
message("This file's directory is ${CMAKE_CURRENT_LIST_DIR}")

# armeabi-v7a requires cpufeatures library
# include(AndroidNdkModules)
# android_ndk_import_module_cpufeatures()

# ----------------------------------------
set(SDL2IMAGE_VENDORED ON)
set(SDL2TTF_VENDORED ON)
set(SDL2MIXER_VENDORED ON)

option(SDL2TTF_HARFBUZZ ON)  # This is set as an option so WebAssembly can disable it
set(SDL2MIXER_FLAC ON)
set(SDL2MIXER_WAVPACK ON)

option(SDL2MIXER_ENABLED ON)  # This is set as an option so WebAssembly can disable it
# ----------------------------------------

# SDL sources are in a subfolder named "SDL"
add_subdirectory(SDL)

# Compilation of companion libraries
add_subdirectory(SDL_image)
add_subdirectory(SDL_ttf)

if (SDL2MIXER_ENABLED)
    add_subdirectory(SDL_mixer)
endif()

# Your game and its CMakeLists.txt are in a subfolder named "src"
#add_subdirectory(src)