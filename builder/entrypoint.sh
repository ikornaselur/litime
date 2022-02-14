#!/usr/bin/env bash

set -eu

if [ -z "${CMD_PATH+x}" ]; then
  export CMD_PATH=""
fi

OUTPUT_DIR="/output"
PROJECT_ROOT="/rust/build/litime"
cd "$PROJECT_ROOT"

# Build Apple
APPLE_OUT="$OUTPUT_DIR/apple"
mkdir -p "$APPLE_OUT"
if ! FILE_LIST=$(/build.sh "$APPLE_OUT" "x86_64-apple-darwin"); then
  echo "::error file=entrypoint.sh::Build failed" >&2
  exit 1
fi

# Build Linux
LINUX_OUT="$OUTPUT_DIR/linux"
mkdir -p "$LINUX_OUT"
if ! FILE_LIST=$(/build.sh "$LINUX_OUT" "x86_64-unknown-linux-musl"); then
  echo "::error file=entrypoint.sh::Build failed" >&2
  exit 1
fi

# Build Windows
WINDOWS_OUT="$OUTPUT_DIR/windows"
mkdir -p "$WINDOWS_OUT"
if ! FILE_LIST=$(/build.sh "$WINDOWS_OUT" "x86_64-pc-windows-gnu"); then
  echo "::error file=entrypoint.sh::Build failed" >&2
  exit 1
fi
