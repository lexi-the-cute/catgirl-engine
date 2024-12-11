#!/bin/sh

# Build Time Autovars
SCRIPT=`realpath "$0"`
SCRIPT_DIR=`dirname "$SCRIPT"`
PROJECT_ROOT=`cd $SCRIPT_DIR/.. && pwd`

rm $SCRIPT_DIR/manifest.json
rm $SCRIPT_DIR/service-worker.js
rm -r $PROJECT_ROOT/_site
rm -r $SCRIPT_DIR/pkg
rm -r $SCRIPT_DIR/resources
