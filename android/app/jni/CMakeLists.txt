cmake_minimum_required(VERSION 3.6)

project(GAME)

# Debug Information
message("This file's directory is ${CMAKE_CURRENT_LIST_DIR}")

# armeabi-v7a requires cpufeatures library
# include(AndroidNdkModules)
# android_ndk_import_module_cpufeatures()

# ----------------------------------------
# Add Libraries CMake Can't Find
# Freetype
#set(SDL2TTF_FREETYPE OFF)
#add_subdirectory(${CMAKE_CURRENT_LIST_DIR}/SDL_ttf/external/freetype)
set(FREETYPE_LIBRARY ${CMAKE_CURRENT_LIST_DIR}/SDL_ttf/external/freetype/build/libfreetype.a)
set(FREETYPE_INCLUDE_DIRS ${CMAKE_CURRENT_LIST_DIR}/SDL_ttf/external/freetype/include)

# Harfbuzz
set(SDL2TTF_HARFBUZZ ON)
#add_subdirectory(${CMAKE_CURRENT_LIST_DIR}/SDL_ttf/external/harfbuzz)
set(harfbuzz_LIBRARY ${CMAKE_CURRENT_LIST_DIR}/SDL_ttf/external/harfbuzz/build/libharfbuzz.a)
set(harfbuzz_INCLUDE_PATH ${CMAKE_CURRENT_LIST_DIR}/SDL_ttf/external/harfbuzz/src)

# Opus
set(SDL2MIXER_OPUS OFF)
#add_subdirectory(${CMAKE_CURRENT_LIST_DIR}/SDL_mixer/external/opus)
set(OpusFile_LIBRARY ${CMAKE_CURRENT_LIST_DIR}/SDL_mixer/external/opus/build/libopus.a)
set(OpusFile_INCLUDE_PATH ${CMAKE_CURRENT_LIST_DIR}/SDL_mixer/external/opus/include)

# OGG
#set(SDL2MIXER_OGG OFF)
#add_subdirectory(${CMAKE_CURRENT_LIST_DIR}/SDL_mixer/external/ogg)
set(OGG_LIBRARY ${CMAKE_CURRENT_LIST_DIR}/SDL_mixer/external/ogg/build/libogg.a)
set(OGG_INCLUDE_DIR ${CMAKE_CURRENT_LIST_DIR}/SDL_mixer/external/ogg/include)

# Flac
set(SDL2MIXER_FLAC OFF)
#add_subdirectory(${CMAKE_CURRENT_LIST_DIR}/SDL_mixer/external/flac)
set(FLAC_LIBRARY ${CMAKE_CURRENT_LIST_DIR}/SDL_mixer/external/flac/build/libflac.a)
set(FLAC_INCLUDE_PATH ${CMAKE_CURRENT_LIST_DIR}/SDL_mixer/external/flac/include)

# WavPack
set(SDL2MIXER_WAVPACK OFF)
#add_subdirectory(${CMAKE_CURRENT_LIST_DIR}/SDL_mixer/external/wavpack)
set(wavpack_LIBRARY ${CMAKE_CURRENT_LIST_DIR}/SDL_mixer/external/wavpack/build/libwavpack.a)
set(wavpack_INCLUDE_PATH ${CMAKE_CURRENT_LIST_DIR}/SDL_mixer/external/wavpack/include)

# XMP
set(SDL2MIXER_MOD_XMP OFF)
#add_subdirectory(${CMAKE_CURRENT_LIST_DIR}/SDL_mixer/external/libxmp)
#set(libxmp_LIBRARY ${CMAKE_CURRENT_LIST_DIR}/SDL_mixer/external/libxmp/build/libxmp.a)
#set(libxmp_INCLUDE_PATH ${CMAKE_CURRENT_LIST_DIR}/SDL_mixer/external/libxmp/include)

# Mod
set(SDL2MIXER_MOD OFF)
# ...

# FluidSynth
set(SDL2MIXER_MIDI_FLUIDSYNTH OFF)
#add_subdirectory(${CMAKE_CURRENT_LIST_DIR}/SDL_mixer/external/fluidsynth)
#set(FluidSynth_LIBRARY ${CMAKE_CURRENT_LIST_DIR}/SDL_mixer/external/fluidsynth/build/libfluidsynth.a)
#set(FluidSynth_INCLUDE_PATH ${CMAKE_CURRENT_LIST_DIR}/SDL_mixer/external/fluidsynth/include)
# ----------------------------------------

# SDL sources are in a subfolder named "SDL"
add_subdirectory(SDL)

# Compilation of companion libraries
add_subdirectory(SDL_image)
add_subdirectory(SDL_ttf)
add_subdirectory(SDL_mixer)

# Your game and its CMakeLists.txt are in a subfolder named "src"
#add_subdirectory(src)