#!/usr/bin/env bash

VERSION="$1"

[[ -n "$VERSION" ]] || exit 1;

echo "============================"
echo "= Publish version $VERSION ="
echo "============================"

echo "Set convert wasm package version"
cargo bump "$VERSION" -k convert || exit 1;

echo "Set caido-convert package version"
cargo bump "$VERSION" -k caido-convert || exit 1;
