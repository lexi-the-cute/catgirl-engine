// Top-level build file where you can add configuration options common to all sub-projects/modules.
plugins {
    // https://mvnrepository.com
    id("com.android.application") version "8.2.2" apply false
    id("com.android.library") version "8.2.2" apply false
    id("org.jetbrains.kotlin.android") version "1.9.22" apply false

    // https://plugins.gradle.org/plugin/com.github.willir.rust.cargo-ndk-android
    id("com.github.willir.rust.cargo-ndk-android") version "0.3.4" apply false
}
