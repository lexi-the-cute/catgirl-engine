#!/bin/sh

# Build Time Autovars
SCRIPT=`realpath "$0"`
SCRIPT_DIR=`dirname "$SCRIPT"`
PROJECT_ROOT=`cd $SCRIPT_DIR/.. && pwd`
HOSTNAME="127.0.0.1"
PORT="4000"

bundle exec jekyll serve --host $HOSTNAME --port $PORT
