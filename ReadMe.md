# Download

* [Itch.io](https://catgirlland.itch.io/catgirl-engine)
* [Google Play](https://play.google.com/store/apps/details?id=land.catgirl.engine)

# Setup Build Environment

## Debian x86_64

```bash
# Update APT
sudo apt update

# Install Required Packages
sudo apt install git gcc libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Run Cargo Environment Setup Script
source "$HOME/.cargo/env"

# Download This Repo
git clone https://github.com/alexisart/CatgirlEngine --recurse-submodules

# Switch To Project Root
cd CatgirlEngine
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

The Android build process can be read from [./.github/workflows/build-android.yml](.github/workflows/build-android.yml)
