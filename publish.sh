#!/bin/bash

echo Stashing all uncommitted data
git add --all
git stash

./version.sh

echo Publishing catgirl-engine-utils
cargo publish -p catgirl-engine-utils --allow-dirty --no-verify

echo Publishing catgirl-engine-client
cargo publish -p catgirl-engine-client --allow-dirty --no-verify

echo Publishing catgirl-engine-server
cargo publish -p catgirl-engine-server --allow-dirty --no-verify

echo Publishing catgirl-engine
cargo publish -p catgirl-engine --allow-dirty --no-verify

echo Resetting utils version
git checkout HEAD -- ./utils/Cargo.toml

echo Resetting client version
git checkout HEAD -- ./client/Cargo.toml

echo Resetting server version
git checkout HEAD -- ./server/Cargo.toml

echo Resetting dependency versions
git checkout HEAD -- ./Cargo.toml
git checkout HEAD -- ./Cargo.lock

git stash pop
