# What's This Repo

This repo is hosted on [Codeberg][codeberg-repo], [Github][github-repo].

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
curl --proto '=https' --tlsv1.2 --silent --show-error --fail --location https://sh.rustup.rs | sh

# Run Cargo Environment Setup Script
source "$HOME/.cargo/env"

# Download Stable Toolchain
rustup default stable

# Download Stable Rust's Source Code
rustup component add rust-src --toolchain stable

# Download This Repo
git clone https://github.com/foxgirl-labs/catgirl-engine

# Switch To Project Root
cd catgirl-engine
```

# Build

## Desktop

```bash
# Compile Program
cargo build
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

If you want to use traces, you can setup your own tracing subscriber if importing as a library.

# Docs

* [Root][catgirl-engine-docs]
* [Client][catgirl-engine-client-docs]
* [Server][catgirl-engine-server-docs]
* [Utils][catgirl-engine-utils-docs]
* [Macros][catgirl-engine-macros-docs]
* [Common][catgirl-engine-commons-docs]

# GPG Keys

## [Main Key 0x1E4A9B8B9E3F25B9C3CD1664C7E8D57343655237][main-key]

```pgp
-----BEGIN PGP PUBLIC KEY BLOCK-----

mDMEZyMQ1xYJKwYBBAHaRw8BAQdAVU5vsu/hdSoYSkcBkmeNEYTB7t/yd/QyCRW8
B5DDjWW0HEFsZXhpcyA8YWxleGlzQGNhdGdpcmwubGFuZD6ImQQTFgoAQRYhBB5K
m4uePyW5w80WZMfo1XNDZVI3BQJnIxDXAhsBBQkJuRAABQsJCAcCAiICBhUKCQgL
AgQWAgMBAh4HAheAAAoJEMfo1XNDZVI3H0gA/ihaveF2G6HepkAclcvpMzitlh25
nvCv9XRvvHJilvYaAP9qSU6HAxAGW/v3Fk0Oq1yH1SpIBLjoeoh46tyACAnnDLgz
BGcjEP4WCSsGAQQB2kcPAQEHQLAVh0ygm8Fd4+51myTYhQpS+9kHdCLuwgURenRQ
EUvUiPUEGBYKACYWIQQeSpuLnj8lucPNFmTH6NVzQ2VSNwUCZyMQ/gIbAgUJCbkQ
AACBCRDH6NVzQ2VSN3YgBBkWCgAdFiEEs9FCtcPYONtYiqKE35ZbzcldW6QFAmcj
EP4ACgkQ35ZbzcldW6Q/DAEAgh50DTETqwoVapehM5IOjOLKF63v9LwhIdPeGHp1
LVIA/2I05TNMgObUUUIcwkA8ahhl/GgYwQR66f+h/5oXfVYJXa0BAKjUV2nrnroL
1DElKRZSOO2gF8ZG6baFfQ2fCWgIPbLVAP4uW+O2WbdWkoEu3ap+NlAahGD28qvy
RrQ4pWLrlFLMDLg4BGcjEVUSCisGAQQBl1UBBQEBB0B7SGkcwhrYHxZ964YCTnRc
UTvgoWjqI3+oU5FQlqffLQMBCAeIfgQYFgoAJhYhBB5Km4uePyW5w80WZMfo1XND
ZVI3BQJnIxFVAhsMBQkJuRAAAAoJEMfo1XNDZVI3as0BANBZuwSrDMjptgsMBoqd
pxoe7Nz4Q7VOFpX83ZHZsGD6AP4nZtbnFt4Iyl2ZjU+1B5liSEalOwwgyU+d8UZV
KJRRCLgzBGcjEXkWCSsGAQQB2kcPAQEHQNhPTV/ZblKRWXlzxH5e2AXJl+GPAAJ5
XFV9DBLtzaceiH4EGBYKACYWIQQeSpuLnj8lucPNFmTH6NVzQ2VSNwUCZyMReQIb
IAUJCbkQAAAKCRDH6NVzQ2VSN5asAQDmHC4z/totwIXQN3XUCic7jyK7PIRg9qGr
0qItURdJcAEA58SsiUqrA4/HwHr9XgsRL6R5Vxs3VldgmFHhVb161QM=
=XU+C
-----END PGP PUBLIC KEY BLOCK-----
```

## [Signing Key 0xE382EE8AE2F4AFB1B18148DFD83603BC74A7BA9C][signing-key]

```pgp
-----BEGIN PGP PUBLIC KEY BLOCK-----

mDMEZyMRxBYJKwYBBAHaRw8BAQdAG5DZ2yFaNmyAp/n9PneeQoGthQg6ZII3kzNn
Ogt98N20JkFsZXhpcyAoU2lnbmluZykgPGFsZXhpc0BjYXRnaXJsLmxhbmQ+iJkE
ExYKAEEWIQTjgu6K4vSvsbGBSN/YNgO8dKe6nAUCZyMRxAIbAwUJCbkQAAULCQgH
AgIiAgYVCgkICwIEFgIDAQIeBwIXgAAKCRDYNgO8dKe6nBopAQD4hW8Z2G5TPOiX
qJ3Ezq8HpvxKEH5v9LvRaEKg3GS2LQD/QVXNFXa0/e+KmeJ1sYDxf8oZNUPkiiV4
kbMEwDqVsQaIdQQQFgoAHRYhBB5Km4uePyW5w80WZMfo1XNDZVI3BQJnIxOSAAoJ
EMfo1XNDZVI3gZgBALRLIpPBxBLrARKgtTxntv6vhgXSVQS6T4SVlkkSZS7VAP9p
n1njsScQPkvSZXRqjYnYfjk9KgzSa6zq8S2z7x2tDIh1BBAWCgAdFiEELKDXRIOl
5ybXwLoacZdd1FBzdMEFAmcjGl4ACgkQcZdd1FBzdMHWgAEA0AIG8GJyiaH7wqGX
YzJXRBWpxc1Iv4I/um/UvRtGfI0A/AuWmJnEL+gUrx+cF1LDzyg3TSKChoCX06cA
oZUeIj0IuDgEZyMR4xIKKwYBBAGXVQEFAQEHQCUquWjoR+WBRErjsFsc02hY9VZ+
nAH8qsIFAt13T4dDAwEIB4h+BBgWCgAmFiEE44LuiuL0r7GxgUjf2DYDvHSnupwF
AmcjEeMCGwwFCQm5EAAACgkQ2DYDvHSnupxR+AEA/rzSj4YRcUcTcNZOiFuTYYH3
bjnt5VwRhrBJZgWFyG0A/j9bpzwURxaLiSxrelcoTiEJ6DFzZcQMKJSp7TvKMU4E
=9s6N
-----END PGP PUBLIC KEY BLOCK-----
```

[github-repo]: https://github.com/foxgirl-labs/catgirl-engine
[codeberg-repo]: https://codeberg.org/alexis/catgirl-engine
[itchio-download]: https://foxgirl-labs.itch.io/catgirl-engine
[catgirl-engine-docs]: https://docs.rs/catgirl-engine/latest/main
[catgirl-engine-client-docs]: https://docs.rs/catgirl-engine-client/latest/catgirl_engine_client
[catgirl-engine-server-docs]: https://docs.rs/catgirl-engine-server/latest/catgirl_engine_server
[catgirl-engine-utils-docs]: https://docs.rs/catgirl-engine-utils/latest/catgirl_engine_utils
[catgirl-engine-macros-docs]: https://docs.rs/catgirl-engine-utils/latest/catgirl_engine_macros
[catgirl-engine-common-docs]: https://docs.rs/catgirl-engine-utils/latest/catgirl_engine_common
[main-key]: http://keyserver.ubuntu.com/pks/lookup?op=vindex&search=0x1E4A9B8B9E3F25B9C3CD1664C7E8D57343655237
[signing-key]: http://keyserver.ubuntu.com/pks/lookup?op=vindex&search=0xE382EE8AE2F4AFB1B18148DFD83603BC74A7BA9C
