I want to make the android build system OS independent and build from fresh git clone to installable apk.

Note: Test in virtual machines to make sure it installs from scratch

# Android

I'll rewrite the Android components later when I have more time. I'll also write proper up to date instructions.

See https://julhe.github.io/posts/building-an-android-app-with-rust-and-sdl2 (Out of Date, But Useful) for now.

https://github.com/libsdl-org/SDL/tree/release-2.26.5

https://developer.android.com/studio/projects/install-ndk

```bash
/home/alexis/Android/Sdk/ndk/25.2.9519653/ndk-build NDK_PROJECT_PATH=. APP_BUILD_SCRIPT=./Android.mk APP_PLATFORM=android-31
```

# Other

```bash
sudo apt install libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev
```