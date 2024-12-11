# Setup Build Environment

## Arch x86_64

```bash
# Update Pacman
sudo pacman -Sy

# Install Required Packages
sudo pacman -S python3 python-pip cython
```

## Python Virtual Environment on Linux

```bash
# Create Virtual Environment
python3 -m venv .venv

# Activate Virtual Environment
source ./.venv/bin/activate

# Upgrade Virtual Environment's Pip
pip install --upgrade pip

# Install Cython and Setup Tools
pip install cython setuptools

# Exit Virtual Environment
deactivate
```

# Build

```bash
# If using Virtual Environment, make sure it's activated

# Copy Cython Header to Working Directory
cp -a ../../../target/binding/catgirl_engine.pxd .

# Compile catgirl_engine Module
./setup.py build_ext --inplace
```

# Run

```bash
# Export Path to libmain.so Library (the game engine)
export LD_LIBRARY_PATH=`realpath ../../../target/debug`

# Run Python Script
python main.py
```
