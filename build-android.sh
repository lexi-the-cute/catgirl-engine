#NOTE: Dont't forget to modify these vars to your setup
JNI_LIBS=./android-project/app/src/main/jniLibs
LIB_NAME=libmain.so
SDL2_LIBS=/home/alexis/.libraries/SDL/libs
BUILD_MODE=release

#copy sdl2 libs into rusts build dir
mkdir -p ./target/aarch64-linux-android/$BUILD_MODE/deps/
mkdir -p ./target/armv7-linux-androideabi/$BUILD_MODE/deps/
mkdir -p ./target/i686-linux-android/$BUILD_MODE/deps/
cp -a $SDL2_LIBS/arm64-v8a/. target/aarch64-linux-android/$BUILD_MODE/deps/
cp -a $SDL2_LIBS/armeabi-v7a/. target/armv7-linux-androideabi/$BUILD_MODE/deps/
cp -a $SDL2_LIBS/x86/. ./target/i686-linux-android/$BUILD_MODE/deps/

#build the libraries
cargo build --target aarch64-linux-android --$BUILD_MODE
cargo build --target armv7-linux-androideabi --$BUILD_MODE
cargo build --target i686-linux-android --$BUILD_MODE

#prepare folders...
rm -rf $JNI_LIBS
mkdir $JNI_LIBS
mkdir $JNI_LIBS/arm64-v8a
mkdir $JNI_LIBS/armeabi-v7a
mkdir $JNI_LIBS/x86

#..and copy the rust library into the android studio project, ready for beeing included into the APK
cp target/aarch64-linux-android/$BUILD_MODE/$LIB_NAME $JNI_LIBS/arm64-v8a/libmain.so
cp target/armv7-linux-androideabi/$BUILD_MODE/$LIB_NAME $JNI_LIBS/armeabi-v7a/libmain.so
cp target/i686-linux-android/$BUILD_MODE/$LIB_NAME $JNI_LIBS/x86/libmain.so 
