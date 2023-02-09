#!/usr/bin/env bash

VERSION="$1"

echo "============================"
echo "= Publish version $VERSION ="
echo "============================"

cargo bump "$VERSION" -k caido-convert

cd caido-convert || exit;
cargo publish || exit;
cd ../

echo "Set convert wasm package version"
cargo bump "$VERSION" -k convert

echo "Build npm package"
just build-convert-release

echo "Publish npm package"
cd wasm-package || exit;
npm publish --access=public
