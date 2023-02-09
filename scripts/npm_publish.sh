#!/usr/bin/env bash

VERSION="$1"

[[ -n "$VERSION" ]] || exit 1;

echo "============================"
echo "= Publish version $VERSION ="
echo "============================"

echo "Bump caido-convert version to $VERSION" 
cargo bump "$VERSION" -k caido-convert || exit 1;

cd caido-convert || exit 1;
echo "Publish caido-convert"
cargo publish --allow-dirty || exit 1;
cd ../

echo "Set convert wasm package version"
cargo bump "$VERSION" -k convert || exit 1;

echo "Build npm package"
just build-convert-release || exit 1;

echo "Publish npm package"
cd wasm-package || exit 1;
npm publish --access=public || exit 1; 
