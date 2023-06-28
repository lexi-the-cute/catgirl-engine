# Download

* [Itch.io](https://catgirlland.itch.io/catgirl-engine)
* [Google Play](https://play.google.com/store/apps/details?id=land.catgirl.engine)

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

# Download Nightly Toolchain
rustup default nightly

# Download Nightly Rust's Source Code
rustup component add rust-src --toolchain nightly

# Download This Repo
git clone https://github.com/alexisart/catgirl-engine --recurse-submodules

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

If you want to display more log messages, on Android, use logcat. On Linux, run `RUST_LOG=debug path/to/engine`.