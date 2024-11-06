#!/bin/bash

CURRENT_DIR=`pwd`
ASSETS_DIR=`pwd`/client/assets
TEXTURES_DIR=$ASSETS_DIR/vanilla/texture
LOGO_DIR=$TEXTURES_DIR/logo

echo Generating Logo PNGs from SVG...

# cd $LOGO_DIR
inkscape -w 48 -h 48 $LOGO_DIR/logo.svg -o $LOGO_DIR/logo-48x48.png
inkscape -w 72 -h 72 $LOGO_DIR/logo.svg -o $LOGO_DIR/logo-72x72.png
inkscape -w 96 -h 96 $LOGO_DIR/logo.svg -o $LOGO_DIR/logo-96x96.png
inkscape -w 144 -h 144 $LOGO_DIR/logo.svg -o $LOGO_DIR/logo-144x144.png
inkscape -w 168 -h 168 $LOGO_DIR/logo.svg -o $LOGO_DIR/logo-168x168.png
inkscape -w 192 -h 192 $LOGO_DIR/logo.svg -o $LOGO_DIR/logo-192x192.png
inkscape -w 256 -h 256 $LOGO_DIR/logo.svg -o $LOGO_DIR/logo-256x256.png
inkscape -w 512 -h 512 $LOGO_DIR/logo.svg -o $LOGO_DIR/logo-512x512.png
inkscape -w 1024 -h 1024 $LOGO_DIR/logo.svg -o $LOGO_DIR/logo-1024x1024.png
