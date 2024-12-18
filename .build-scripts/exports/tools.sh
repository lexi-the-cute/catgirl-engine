#!/bin/env bash
# Setup Bash Safety Checks
set -eo pipefail

# Setup for Build Time Autovars
if [ -z "$REALPATH_EXE" ]; then
    REALPATH_EXE=`which realpath`  # /usr/bin/realpath
fi

if [ -z "$DIRNAME_EXE" ]; then
    DIRNAME_EXE=`which dirname`  # /usr/bin/dirname
fi

# Build Time Autovars
SCRIPT=`$REALPATH_EXE "$0"`
SCRIPT_DIR=`$DIRNAME_EXE "$SCRIPT"`
PROJECT_ROOT=`$REALPATH_EXE $SCRIPT_DIR/../..`

# cd $PROJECT_ROOT

# Shell Command Locations
if [ -z "$ROOT_PATH" ]; then
    if [ "$WORKSPACE" ]; then
        # If workspace is specified like on CI, then stick on home directory
        ROOT_PATH=$HOME
    else
        # Else, keep all tools local
        ROOT_PATH=$PROJECT_ROOT
    fi
fi

if [ -z "$TOOLS_PATH" ]; then
    TOOLS_PATH=$ROOT_PATH/.tools
fi

case :$PATH:
  in *:$TOOLS_PATH:*) ;;
     *) PATH=$TOOLS_PATH:$PATH ;;
esac

case :$PATH:
  in *:$TOOLS_PATH/butler:*) ;;
     *) PATH=$TOOLS_PATH/butler:$PATH ;;
esac

export PATH=$PATH
set +eo pipefail
