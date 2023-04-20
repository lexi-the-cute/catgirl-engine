# This exists as a temporary resource for writing your own compilation scripts

reset
clear

cd /home/alexis/Desktop/game

# ...
cargo clean

# NOTE: If you change the Android API to call, change the version numbers in ./.cargo/config.toml too
# Compile SDL For Android (APP_PLATFORM is Version Of Android API To Compile For)
/home/alexis/Android/Sdk/ndk/25.2.9519653/ndk-build NDK_PROJECT_PATH="./android/app/jni/SDL" APP_BUILD_SCRIPT="./android/app/jni/SDL/Android.mk" APP_PLATFORM="android-21"
/home/alexis/Android/Sdk/ndk/25.2.9519653/ndk-build NDK_PROJECT_PATH="./android/app/jni/SDL_image" APP_BUILD_SCRIPT="./android/app/jni/SDL_image/Android.mk" APP_PLATFORM="android-21"
# /home/alexis/Android/Sdk/ndk/25.2.9519653/ndk-build NDK_PROJECT_PATH="./android/app/jni/freetype" APP_BUILD_SCRIPT="./android/app/jni/freetype/Android.mk" APP_PLATFORM="android-21"
# /home/alexis/Android/Sdk/ndk/25.2.9519653/ndk-build NDK_PROJECT_PATH="./android/app/jni/harfbuzz" APP_BUILD_SCRIPT="./android/app/jni/harfbuzz/Android.mk" APP_PLATFORM="android-21"
# /home/alexis/Android/Sdk/ndk/25.2.9519653/ndk-build NDK_PROJECT_PATH="./android/app/jni/SDL_ttf" APP_BUILD_SCRIPT="./android/app/jni/SDL_ttf/Android.mk" APP_PLATFORM="android-21"

# Make Directories For Dropping SDL Library Into
mkdir -p ./target/aarch64-linux-android/release/deps
mkdir -p ./target/armv7-linux-androideabi/release/deps
mkdir -p ./target/i686-linux-android/release/deps
mkdir -p ./target/x86_64-linux-android/release/deps
# mkdir -p ./target/x86_64-linux-laptop/release/deps

# Copy SDL Library Over To Corresponding Directories
cp -a ./android/app/jni/SDL/libs/arm64-v8a/libSDL2.so ./target/aarch64-linux-android/release/deps/libSDL2.so
cp -a ./android/app/jni/SDL/libs/armeabi-v7a/libSDL2.so ./target/armv7-linux-androideabi/release/deps/libSDL2.so
cp -a ./android/app/jni/SDL/libs/x86/libSDL2.so ./target/i686-linux-android/release/deps/libSDL2.so
cp -a ./android/app/jni/SDL/libs/x86_64/libSDL2.so ./target/x86_64-linux-android/release/deps/libSDL2.so

# Copy SDL Image Library Over To Corresponding Directories
cp -a ./android/app/jni/SDL_image/libs/arm64-v8a/libSDL2_image.so ./target/aarch64-linux-android/release/deps/libSDL2_image.so
cp -a ./android/app/jni/SDL_image/libs/armeabi-v7a/libSDL2_image.so ./target/armv7-linux-androideabi/release/deps/libSDL2_image.so
cp -a ./android/app/jni/SDL_image/libs/x86/libSDL2_image.so ./target/i686-linux-android/release/deps/libSDL2_image.so
cp -a ./android/app/jni/SDL_image/libs/x86_64/libSDL2_image.so ./target/x86_64-linux-android/release/deps/libSDL2_image.so

# Build Engine As Library
cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
cargo build --target i686-linux-android --release
cargo build --target x86_64-linux-android --release
# cargo build --target x86_64-linux-laptop --release

# Make Directories For Storing Engine In Android App
rm -rf ./android/app/src/main/jniLibs
mkdir ./android/app/src/main/jniLibs
mkdir ./android/app/src/main/jniLibs/arm64-v8a
mkdir ./android/app/src/main/jniLibs/armeabi-v7a
mkdir ./android/app/src/main/jniLibs/x86
mkdir ./android/app/src/main/jniLibs/x86_64

# Copy Libraries To Android App
cp ./target/aarch64-linux-android/release/deps/*.so ./android/app/src/main/jniLibs/arm64-v8a/
cp ./target/armv7-linux-androideabi/release/deps/*.so ./android/app/src/main/jniLibs/armeabi-v7a/
cp ./target/i686-linux-android/release/deps/*.so ./android/app/src/main/jniLibs/x86/
cp ./target/x86_64-linux-android/release/deps/*.so ./android/app/src/main/jniLibs/x86_64/

# Copy Engine Over To Android App
# cp ./target/aarch64-linux-android/release/libmain.so ./android/app/src/main/jniLibs/arm64-v8a/libmain.so
# cp ./target/armv7-linux-androideabi/release/libmain.so ./android/app/src/main/jniLibs/armeabi-v7a/libmain.so
# cp ./target/i686-linux-android/release/libmain.so ./android/app/src/main/jniLibs/x86/libmain.so
# cp ./target/x86_64-linux-android/release/libmain.so ./android/app/src/main/jniLibs/x86_64/libmain.so

# Switch Over To Android App Directory
cd android

# ...
./gradlew clean

# Build Android Apk
./gradlew assembleDebug

# Switch Over To Project Root
cd ..

# Find Compiled Android App
echo You can find your apk in ./android/app/build/outputs/apk/debug
ls -liallh ./android/app/build/outputs/apk/debug

# ...
adb install ./android/app/build/outputs/apk/debug/app-debug.apk