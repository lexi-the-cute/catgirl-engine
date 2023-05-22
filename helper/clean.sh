cd `dirname "$0"`
cd ..
export WORKSPACE=`pwd`
rm -rf $WORKSPACE/android/app/src/main/jniLibs
rm -rf $WORKSPACE/android/app/jni/build
rm -rf $WORKSPACE/android/app/jni/*/build
rm -rf $WORKSPACE/android/app/jni/*/external/*/build
rm -rf $WORKSPACE/build
rm -rf $WORKSPACE/target

cargo clean

cd android
./gradlew clean