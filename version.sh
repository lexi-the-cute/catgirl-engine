#!/bin/bash

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
