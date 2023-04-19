# Setup Build Environment

## Debian x86_64

```bash
# Update APT
sudo apt update

# Install Required Packages
sudo apt install git gcc libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Run Cargo Environment Setup Script
source "$HOME/.cargo/env"

# Download This Repo
git clone https://github.com/alexisart/game --recurse-submodules

# Switch To Project Root
cd game
```

### Android Support

```bash
# Update APT
sudo apt update

# Install Required Packages
sudo apt install unzip openjdk-17-jre-headless

# Make Tools Directory
mkdir tools

# Switch To Tools Directory
cd tools

# Download Android NDK
wget https://dl.google.com/android/repository/android-ndk-r25c-linux.zip

# Unzip Android NDK
unzip android-ndk-r25c-linux.zip

# Download Android SDK
wget https://dl.google.com/android/repository/commandlinetools-linux-9477386_latest.zip

# Unzip Android SDK
unzip commandlinetools-linux-9477386_latest.zip

# Download Desired Android API
./cmdline-tools/bin/sdkmanager --install "platforms;android-21" --sdk_root=.

# Switch Back To Project Root
cd ..

# Add Build Targets For Android
rustup target add armv7-linux-androideabi
rustup target add aarch64-linux-android
rustup target add i686-linux-android
rustup target add x86_64-linux-android
```

# Build

## Debian x86_64

```bash
# Compile Program
cargo build
```

### Android Support

```bash
# NOTE: If you change the Android API to call, change the version numbers in ./.cargo/config.toml too
# Compile SDL For Android (APP_PLATFORM is Version Of Android API To Compile For)
# TODO: Add SDL_image and SDL_ttf to build includes
# ./tools/android-ndk-r25c/ndk-build ... SDL_image
# ./tools/android-ndk-r25c/ndk-build ... SDL_ttf
./tools/android-ndk-r25c/ndk-build NDK_PROJECT_PATH="./android/app/jni/SDL" APP_BUILD_SCRIPT="./android/app/jni/SDL/Android.mk" APP_PLATFORM="android-21"
./tools/android-ndk-r25c/ndk-build NDK_PROJECT_PATH="./android/app/jni/SDL_image" APP_BUILD_SCRIPT="./android/app/jni/SDL_image/Android.mk" APP_PLATFORM="android-21"
# ./tools/android-ndk-r25c/ndk-build NDK_PROJECT_PATH="./android/app/jni/freetype" APP_BUILD_SCRIPT="./android/app/jni/freetype/Android.mk" APP_PLATFORM="android-21"
# ./tools/android-ndk-r25c/ndk-build NDK_PROJECT_PATH="./android/app/jni/harfbuzz" APP_BUILD_SCRIPT="./android/app/jni/harfbuzz/Android.mk" APP_PLATFORM="android-21"
# ./tools/android-ndk-r25c/ndk-build NDK_PROJECT_PATH="./android/app/jni/SDL_ttf" APP_BUILD_SCRIPT="./android/app/jni/SDL_ttf/Android.mk" APP_PLATFORM="android-21"

# Make Directories For Dropping SDL Library Into
mkdir -p ./target/aarch64-linux-android/release/deps
mkdir -p ./target/armv7-linux-androideabi/release/deps
mkdir -p ./target/i686-linux-android/release/deps
mkdir -p ./target/x86_64-linux-android/release/deps

# Copy SDL Library Over To Corresponding Directories
cp -a ./android/app/jni/SDL/libs/arm64-v8a/. ./target/aarch64-linux-android/release/deps
cp -a ./android/app/jni/SDL/libs/armeabi-v7a/. ./target/armv7-linux-androideabi/release/deps
cp -a ./android/app/jni/SDL/libs/x86/. ./target/i686-linux-android/release/deps
cp -a ./android/app/jni/SDL/libs/x86_64/. ./target/x86_64-linux-android/release/deps

# Build Engine As Library
cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
cargo build --target i686-linux-android --release
cargo build --target x86_64-linux-android --release

# Make Directories For Storing Engine In Android App
rm -rf ./android/app/src/main/jniLibs
mkdir ./android/app/src/main/jniLibs
mkdir ./android/app/src/main/jniLibs/arm64-v8a
mkdir ./android/app/src/main/jniLibs/armeabi-v7a
mkdir ./android/app/src/main/jniLibs/x86
mkdir ./android/app/src/main/jniLibs/x86_64

# Copy Engine Over To Android App
cp ./target/aarch64-linux-android/release/libmain.so ./android/app/src/main/jniLibs/arm64-v8a/libmain.so
cp ./target/armv7-linux-androideabi/release/libmain.so ./android/app/src/main/jniLibs/armeabi-v7a/libmain.so
cp ./target/i686-linux-android/release/libmain.so ./android/app/src/main/jniLibs/x86/libmain.so
cp ./target/x86_64-linux-android/release/libmain.so ./android/app/src/main/jniLibs/x86_64/libmain.so

# Set SDK Location
cd ./tools && export ANDROID_HOME="`pwd`" && cd ..

# Switch Over To Android App Directory
cd android

# Build Android Apk
./gradlew assembleDebug

# Switch Over To Project Root
cd ..

# Find Compiled Android App
echo You can find your apk in ./android/app/build/outputs/apk/debug
ls -liallh ./android/app/build/outputs/apk/debug
```

# Run

## Debian x86_64

```bash
# Compile and Run Program
cargo run
```