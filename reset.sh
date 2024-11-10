#!/bin/bash

SCRIPT=`realpath "$0"`
SCRIPT_DIR=`dirname "$SCRIPT"`
PROJECT_ROOT=$SCRIPT_DIR

echo "This will reset everything that isn't committed..."

[[ "$(read -e -p 'Continue? [y/N]> '; echo $REPLY)" == [Yy]* ]]

cd $PROJECT_ROOT

echo "Cleaning rust build data..."
cargo clean

echo "Cleaning android build data..."
cd $PROJECT_ROOT/android && $PROJECT_ROOT/gradlew clean

echo "Unstaging all files..."
git rm -rf --cached $PROJECT_ROOT

echo "Cleaning .gitignored files..."
git clean -dfX

echo "Resetting repo to HEAD..."
git reset --hard
