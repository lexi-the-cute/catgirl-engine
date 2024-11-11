#!/bin/bash

# Build Time Autovars
SCRIPT=`realpath "$0"`
SCRIPT_DIR=`dirname "$SCRIPT"`
PROJECT_ROOT=$SCRIPT_DIR

cd $PROJECT_ROOT
pre-commit autoupdate
