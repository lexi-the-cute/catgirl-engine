<!-- # What's This About -->

# What's This Repo

<!-- Future explanation of repo here... -->

This repo is hosted on [my Forgejo instance][forgejo-repo], [Codeberg][codeberg-repo], [Github][github-repo].

# Download

* [Itch.io][itchio-download]
* [Google Play][google-play-download]

# Setup Build Environment

## Debian x86_64

```bash
# Update APT
sudo apt update

# Install Required Packages
sudo apt install git gcc

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Run Cargo Environment Setup Script
source "$HOME/.cargo/env"

# Download Stable Toolchain
rustup default stable

# Download Stable Rust's Source Code
rustup component add rust-src --toolchain stable

# Download This Repo
git clone https://github.com/lexi-the-cute/catgirl-engine

# Switch To Project Root
cd catgirl-engine
```

# Build

## Debian x86_64

```bash
# Compile Program
cargo run --release

# For Installing As Deb
cargo install cargo-deb
cargo deb --install
catgirl-engine
```

## Android

```bash
# Assuming In Project Root "catgirl-engine" From Debian x86_64

# Install Java If Not Already Installed
sudo apt -y install openjdk-17-jre-headless

# Add Build Targets Once
rustup target add armv7-linux-androideabi
rustup target add aarch64-linux-android
rustup target add i686-linux-android
rustup target add x86_64-linux-android

# Install Cargo-NDK Once
cargo install cargo-ndk

# Workaround Cargo Gradle Plugin Bug Once
touch android/local.properties

# Build Android APK
cd android
./gradlew assembleDebug

# Copy Android APK To Project Root
cp app/build/outputs/apk/debug/*.apk ..
```

## Others

Other build process can be read from the files stored in [./.github/workflows](.github/workflows/). Most build files build on x86_64 Ubuntu with the exception of Mac OSX which builds on 64 Bit OSX.

# Running

If you want to display more log messages, on Android, use logcat. On Linux, run `RUST_LOG=catgirl_engine=debug path/to/engine` or `RUST_LOG=debug path/to/engine`.

If you want to use traces, you can either setup your own tracing subscriber if importing as a library, or turn on the tracing-subscriber feature (for either the binary or library).

[forgejo-repo]: https://git.catgirl.land/catgirl-land/catgirl-engine
[github-repo]: https://github.com/lexi-the-cute/catgirl-engine
[codeberg-repo]: https://codeberg.org/alexis/catgirl-engine
[itchio-download]: https://fomxgorl.itch.io/catgirl-engine
[google-play-download]: https://play.google.com/store/apps/details?id=land.catgirl.engine
