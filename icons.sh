#!/bin/bash

# Build Time Autovars
SCRIPT=`realpath "$0"`
SCRIPT_DIR=`dirname "$SCRIPT"`
PROJECT_ROOT=$SCRIPT_DIR

ASSETS_DIR=$PROJECT_ROOT/client/assets
TEXTURES_DIR=$ASSETS_DIR/vanilla/texture
LOGO_DIR=$TEXTURES_DIR/logo

echo Generating Logo PNGs from SVG...

# cd $LOGO_DIR
inkscape --export-background=#505050 -w 48 -h 48 $LOGO_DIR/logo.svg -o $LOGO_DIR/logo-48x48.png
inkscape --export-background=#505050 -w 72 -h 72 $LOGO_DIR/logo.svg -o $LOGO_DIR/logo-72x72.png
inkscape --export-background=#505050 -w 96 -h 96 $LOGO_DIR/logo.svg -o $LOGO_DIR/logo-96x96.png
inkscape --export-background=#505050 -w 144 -h 144 $LOGO_DIR/logo.svg -o $LOGO_DIR/logo-144x144.png
inkscape --export-background=#505050 -w 168 -h 168 $LOGO_DIR/logo.svg -o $LOGO_DIR/logo-168x168.png
inkscape --export-background=#505050 -w 192 -h 192 $LOGO_DIR/logo.svg -o $LOGO_DIR/logo-192x192.png
inkscape --export-background=#505050 -w 256 -h 256 $LOGO_DIR/logo.svg -o $LOGO_DIR/logo-256x256.png
inkscape --export-background=#505050 -w 512 -h 512 $LOGO_DIR/logo.svg -o $LOGO_DIR/logo-512x512.png
inkscape --export-background=#505050 -w 1024 -h 1024 $LOGO_DIR/logo.svg -o $LOGO_DIR/logo-1024x1024.png
