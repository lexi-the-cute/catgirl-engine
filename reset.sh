#!/bin/bash

SCRIPT=`realpath "$0"`
SCRIPT_DIR=`dirname "$SCRIPT"`
PROJECT_ROOT=$SCRIPT_DIR

echo "Project Root: $PROJECT_ROOT"
echo "This will reset everything that isn't committed..."

if [[ "$(read -e -p 'Continue? [y/N]> '; echo $REPLY)" == [Yy]* ]]; then
    cd $PROJECT_ROOT

    echo "Cleaning rust build data..."
    cargo clean

    echo "Cleaning android build data..."
    cd $PROJECT_ROOT/android && $PROJECT_ROOT/android/gradlew clean
    cd $PROJECT_ROOT

    echo "Unstaging all files..."
    git reset

    echo "Cleaning .gitignored files..."
    git clean -dfX

    echo "Resetting repo to HEAD..."
    git reset --hard
fi
