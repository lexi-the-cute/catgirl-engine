#!/bin/bash

BRANCH="main"

SCRIPT=`realpath "$0"`
SCRIPT_DIR=`dirname "$SCRIPT"`
PROJECT_ROOT=$SCRIPT_DIR

echo "Project Root: $PROJECT_ROOT"
echo "This will configure your repo and RESET everything that isn't committed..."
echo 'This will NOT install git pre-commit hooks. You need to run `pre-commit install` afterward...'
echo "This will NOT setup your build tools either, run \`$PROJECT_ROOT/install-tools.sh\` afterward too..."

if [[ "$(read -e -p 'Continue? [y/N]> '; echo $REPLY)" == [Yy]* ]]; then
    cd $PROJECT_ROOT

    echo "Updating repo to index version 4"
    git update-index --index-version 4

    echo "Configuring local git config..."
    git config feature.manyFiles true
    git config core.fsmonitor true
    git config core.untrackedcache true
    git config core.commitgraph true
    git config fetch.writeCommitGraph true

    echo "Installing Git LFS and setting up locks..."
    git lfs install
    git config lfs.https://codeberg.org/alexis/catgirl-engine.git/info/lfs.locksverify true
    git config lfs.https://github.com/foxgirl-labs/catgirl-engine.git/info/lfs.locksverify true

    echo "Unstaging all files..."
    git reset

    echo "Cleaning .gitignored files..."
    git clean -dfX

    echo "Resetting repo to HEAD..."
    git reset --hard

    echo "Checking out $BRANCH branch..."
    git checkout $BRANCH
fi
