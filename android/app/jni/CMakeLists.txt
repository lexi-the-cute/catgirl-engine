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

set(SDL2TTF_HARFBUZZ OFF)
set(SDL2MIXER_FLAC OFF)
set(SDL2MIXER_WAVPACK OFF)
# ----------------------------------------

# SDL sources are in a subfolder named "SDL"
add_subdirectory(SDL)

# Compilation of companion libraries
add_subdirectory(SDL_image)
add_subdirectory(SDL_ttf)
#add_subdirectory(SDL_mixer)

# Your game and its CMakeLists.txt are in a subfolder named "src"
#add_subdirectory(src)