#!/bin/bash

echo Stashing all uncommitted data
git add --all
git stash

export VERSION=`cat ./Cargo.toml | grep '^version' | head -n1 | cut -d'"' -f2 | tr -d '\n'`
echo Publish catgirl-engine v$VERSION

echo Replacing version number in catgirl-engine-utils with catgirl-engine version
sed -i "s/^version = \"[0-9.]*\"/version = \"$VERSION\"/" ./utils/Cargo.toml

echo Replacing version number in catgirl-engine-client with catgirl-engine version
sed -i "s/^version = \"[0-9.]*\"/version = \"$VERSION\"/" ./client/Cargo.toml
sed -i "s/^utils = { version = \"[0-9.]*\"/utils = { version = \"$VERSION\"/" ./client/Cargo.toml

echo Replacing version number in catgirl-engine-server with catgirl-engine version
sed -i "s/^version = \"[0-9.]*\"/version = \"$VERSION\"/" ./server/Cargo.toml
sed -i "s/^utils = { version = \"[0-9.]*\"/utils = { version = \"$VERSION\"/" ./server/Cargo.toml

echo Replacing version number in catgirl-engine dependencies with catgirl-engine version
sed -i "s/^utils = { version = \"[0-9.]*\"/utils = { version = \"$VERSION\"/" ./Cargo.toml
sed -i "s/^client = { version = \"[0-9.]*\"/client = { version = \"$VERSION\"/" ./Cargo.toml
sed -i "s/^server = { version = \"[0-9.]*\"/server = { version = \"$VERSION\"/" ./Cargo.toml

echo Publishing catgirl-engine-utils
cargo publish -p catgirl-engine-utils --allow-dirty

echo Publishing catgirl-engine-client
cargo publish -p catgirl-engine-client --allow-dirty

echo Publishing catgirl-engine-server
cargo publish -p catgirl-engine-server --allow-dirty

echo Publishing catgirl-engine
cargo publish -p catgirl-engine --allow-dirty

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
