wasm-package-path := "./wasm-package"
wasm-crate := "./convert"

default:
    @just --list

build-convert-release:
  wasm-pack build --out-dir ../{{wasm-package-path}} --target=bundler ./{{wasm-crate}} --scope caido
  wasm-opt -Os -o ./{{wasm-package-path}}/convert_bg.wasm ./{{wasm-package-path}}/convert_bg.wasm

build-wasm-dev:
  wasm-pack build --out-dir ../{{wasm-package-path}} --target=bundler ./{{wasm-crate}} --dev
  yarn replace-in-file "/\"module\":/" "\"type\": \"module\", \"main\":" "{{wasm-package-path}}/package.json" --isRegex
  yarn prettier --write "{{wasm-package-path}}/package.json"

wasm-tests: build-wasm-dev
  yarn workspace web-tests test

clippy:
  cargo clippy -p caido-convert
  cargo clippy -p convert

format:
  cargo +nightly fmt

format-check:
  cargo +nightly fmt --check

test: 
  cargo test -p caido-convert 
  just wasm-tests

coverage:
  cargo llvm-cov -p caido-convert --html --open --ignore-filename-regex errors

lcov:
  cargo llvm-cov -p caido-convert --ignore-filename-regex errors --lcov --output-path lcov.info
  
