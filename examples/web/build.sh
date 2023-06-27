#!/bin/sh

# Install Wasm-Pack
# cargo install wasm-pack

if [[ -f ".gitignore.bak" ]]; then
    rm .gitignore.bak
fi
cp .gitignore .gitignore.bak

# Build Catgirl Engine
wasm-pack build --target web --no-pack --no-typescript --out-name main --out-dir "`pwd`" --release

mv main_bg.wasm main.wasm
rm .gitignore
mv .gitignore.bak .gitignore

# Start Server
python3 start.py