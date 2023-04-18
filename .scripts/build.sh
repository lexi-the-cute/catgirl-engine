# TODO: Check if submodules were initialized and initialize them here before build if not
# TODO: Setup local.properties

./clean.sh

cargo build
(cd android/app/jni/SDL && /home/alexis/Android/Sdk/ndk/25.2.9519653/ndk-build NDK_PROJECT_PATH=. APP_BUILD_SCRIPT=Android.mk APP_PLATFORM=android-31)
android.sh

echo You can find your apk in ./android/app/build/outputs/apk/debug
ls -liallh android/app/build/outputs/apk/debug
