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
git clone https://github.com/alexisart/game --recurse-submodules

# Switch To Project Root
cd game
```

# Build

## Debian x86_64

```bash
# Compile Program
cargo build
```

## Android

The Android build process can be read from [./.github/workflows/build-android.yml](.github/workflows/build-android.yml)