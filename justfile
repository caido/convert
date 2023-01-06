default:
    @just --list

build-wasm-convert:
  wasm-pack build --out-dir ../wasm-convert --target=bundler ./wasm-crate
  yarn replace-in-file "/\"module\":/" "\"type\": \"module\", \"main\":" wasm-convert/package.json --isRegex
  yarn prettier --write wasm-convert/package.json

wasm-tests: build-wasm-convert
  yarn workspace web test
