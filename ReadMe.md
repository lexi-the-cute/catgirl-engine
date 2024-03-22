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

# GPG Keys

## [Main Key 0x20E0635864445A177F8F7C0C6141FD27892AE9B4][main-key]

```pgp
-----BEGIN PGP PUBLIC KEY BLOCK-----

mDMEZVb0bRYJKwYBBAHaRw8BAQdAr/5xsAMr4nh8TLeSqlxuYVSdTDpAyp0IpxmG
fr34+c60HEFsZXhpcyA8YWxleGlzQGNhdGdpcmwubGFuZD6IkAQTFggAOBYhBCDg
Y1hkRFoXf498DGFB/SeJKum0BQJlVvRtAhsBBQsJCAcCBhUKCQgLAgQWAgMBAh4B
AheAAAoJEGFB/SeJKum01+UBALMVSVT9nmeWgRtrf3haWN0P5leiurN8FJvih0Hh
ofs9AP9CD1Jv6I3f5c/xnRS0Wp8MaGFMvP0Iyi9ZC8DD86GACrgzBGVW9JAWCSsG
AQQB2kcPAQEHQMCthm/wQ/Ksat5X4zmMYJ9ebJvhFFQKguJ6LDu/J0tKiPUEGBYI
ACYWIQQg4GNYZERaF3+PfAxhQf0niSrptAUCZVb0kAIbAgUJAeEzgACBCRBhQf0n
iSrptHYgBBkWCAAdFiEEE1EfbwiAqr0HqhA10HU9Q/PHqUIFAmVW9JAACgkQ0HU9
Q/PHqUIRugD+N2rrn51Aj39zZplMMjDgIjdfJ/MKcolQAg3VdNLRDusA/2/35T2t
NATQPpIv2bQSghn8YWq73+1uFUkumfkLF7oPFPwBAPx23M1gsBYiYRn0sXahyubt
9ynKw75OVdKyOHCHzNEdAQCjMz16Io+OwSVksqjnVQpbi55jWAj9KuK2MqLgEUPl
C7g4BGVW9L8SCisGAQQBl1UBBQEBB0D2I482Q12nobygTwIq5PWJQdxvzSrYYu20
8gmeRbxkdAMBCAeIfgQYFggAJhYhBCDgY1hkRFoXf498DGFB/SeJKum0BQJlVvS/
AhsMBQkB4TOAAAoJEGFB/SeJKum0iBYA/jMGJXN8MWaxxoOFEUmdxAPuMRnYA26l
IbnOYsGXiqI+AQD7kOstyDIPBR5LrNw6DEIImZkNQNlQpQ4RfJio3+CAAbgzBGVW
9MsWCSsGAQQB2kcPAQEHQA9OjvcEIu00alJIZGUSO1yel/fEQznG7o5zTTK2KIPz
iH4EGBYIACYWIQQg4GNYZERaF3+PfAxhQf0niSrptAUCZVb0ywIbIAUJAeEzgAAK
CRBhQf0niSrptJPTAQCqoYabT6o/9Cl+W0UglUuszSqv48ReLsD6YSQIrr1vPgEA
2peTLx2rNhghMf4MCQq+3ya6wfxnnAPSuqqDInxVdwA=
=+HCr

-----END PGP PUBLIC KEY BLOCK-----
```

## [Signing Key 0x2CA0D74483A5E726D7C0BA1A71975DD4507374C1][signing-key]

```pgp
-----BEGIN PGP PUBLIC KEY BLOCK-----

mDMEZfvPrxYJKwYBBAHaRw8BAQdAEC5CDYx2YHy+Y4sHXPcEcD0a4m4IaixdD6D8
0pqOKaq0JkFsZXhpcyAoU2lnbmluZykgPGFsZXhpc0BjYXRnaXJsLmxhbmQ+iJwE
ExYKAEQCGwMFCQWjmoAFCwkIBwICIgIGFQoJCAsCBBYCAwECHgcCF4AWIQQsoNdE
g6XnJtfAuhpxl13UUHN0wQUCZfvP4AIZAQAKCRBxl13UUHN0wQ0xAP9pK6w4qmTa
flz2PijnJmNvl0zO+0FmmM1YLDspvcrD1AD+Oi4bdpAgwyS9xcXIGq+5+IcDUcJk
MeaAmKte0NHVyQ6IdQQQFgoAHRYhBCDgY1hkRFoXf498DGFB/SeJKum0BQJl/QUQ
AAoJEGFB/SeJKum0LlAA/jYVOHeuXGxtKdw5pdZDkHqpNczZQDhY784vQrofp688
AQDvS6Kw82pGHltobmCdAwJPA06hDSYwlwtI+m25QYfzCLg4BGX7z68SCisGAQQB
l1UBBQEBB0DvJLTaa2ew55oxSnhrF0R10ssM7wxOfAf5EtKa8/1ZRgMBCAeIfgQY
FgoAJhYhBCyg10SDpecm18C6GnGXXdRQc3TBBQJl+8+vAhsMBQkFo5qAAAoJEHGX
XdRQc3TBNLIA/2VY6mQ9cvR5Zk7Lh02dGuVtOqar3LyTP3ue4CISrDagAQCNXVAR
SS1k4L1wKTICYf+SL07C8klAKNF4llhUvBmfAQ==
=coKj

-----END PGP PUBLIC KEY BLOCK-----
```

[forgejo-repo]: https://git.catgirl.land/catgirl-land/catgirl-engine
[github-repo]: https://github.com/lexi-the-cute/catgirl-engine
[codeberg-repo]: https://codeberg.org/alexis/catgirl-engine
[itchio-download]: https://fomxgorl.itch.io/catgirl-engine
[catgirl-engine-docs]: https://docs.rs/catgirl-engine/latest/main
[catgirl-engine-client-docs]: https://docs.rs/catgirl-engine-client/latest/catgirl_engine_client
[catgirl-engine-server-docs]: https://docs.rs/catgirl-engine-server/latest/catgirl_engine_server
[catgirl-engine-utils-docs]: https://docs.rs/catgirl-engine-utils/latest/catgirl_engine_utils
[main-key]: http://keyserver.ubuntu.com/pks/lookup?op=vindex&search=0x20E0635864445A177F8F7C0C6141FD27892AE9B4
[signing-key]: http://keyserver.ubuntu.com/pks/lookup?op=vindex&search=0x2CA0D74483A5E726D7C0BA1A71975DD4507374C1
