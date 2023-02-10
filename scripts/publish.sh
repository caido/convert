#!/usr/bin/env bash

echo "Publish caido-convert"
cd caido-convert || exit 1;
cargo publish --allow-dirty || exit 1;
cd ../

echo "Build npm package"
just build-convert-release || exit 1;

echo "Publish npm package"
cd wasm-package || exit 1;
npm publish --access=public || exit 1; 

echo "Packages are publish"
