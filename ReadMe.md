# What's This Repo

This repo is hosted on [my Forgejo instance][forgejo-repo], [Codeberg][codeberg-repo], [Github][github-repo].

# Why Yet Another Engine

This game engine is designed around moddability. This will allow people to make different games based on it, which should hypothetically all be compatible with each other. In order to enforce the spirit of the design, the game built into the engine will itself be a mod.

The engine will also allow transferring items and other entities between single player and multiplayer including transferring modded items from a modded server to a vanilla client.

Hypothetically, if a person develops a different game, like a horror game, it should be possible to join the server for that game from any other game made on the engine including the built in game.

I'm also considering the idea of inter-server communication, but for now, communication happens through the client transferring items and entities.

# Download

* [Itch.io][itchio-download]

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

## Desktop

```bash
# Compile Program
cargo run
```

## Android

```bash
# Assuming In Project Root "catgirl-engine" From Debian x86_64

# Install Java If Not Already Installed
sudo apt -y install openjdk-17-jre-headless

# Add Build Targets Once
rustup target add armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android

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

On Linux, you can view more log messages by running `RUST_LOG=catgirl_engine=debug path/to/engine` or `RUST_LOG=debug path/to/engine`.

On Android, you can view more log messages with ADB by running `adb logcat -v tag,color -s CatgirlEngineApp CatgirlEngine:D`. You can clear the log by running `adb logcat -c`.

If you want to use traces, you can either setup your own tracing subscriber if importing as a library, or turn on the tracing-subscriber feature (for either the binary or library).

# Docs

* [Root][catgirl-engine-docs]
* [Client][catgirl-engine-client-docs]
* [Server][catgirl-engine-server-docs]
* [Utils][catgirl-engine-utils-docs]

[forgejo-repo]: https://git.catgirl.land/catgirl-land/catgirl-engine
[github-repo]: https://github.com/lexi-the-cute/catgirl-engine
[codeberg-repo]: https://codeberg.org/alexis/catgirl-engine
[itchio-download]: https://fomxgorl.itch.io/catgirl-engine
[catgirl-engine-docs]: https://docs.rs/catgirl-engine/latest/main
[catgirl-engine-client-docs]: https://docs.rs/catgirl-engine-client/latest/catgirl_engine_client
[catgirl-engine-server-docs]: https://docs.rs/catgirl-engine-server/latest/catgirl_engine_server
[catgirl-engine-utils-docs]: https://docs.rs/catgirl-engine-utils/latest/catgirl_engine_utils
