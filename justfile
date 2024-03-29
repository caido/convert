wasm-package-path := "./wasm-package"
wasm-crate := "./convert"

default:
  @just --list

build-convert-release:
  wasm-pack build --out-name index --out-dir ../{{wasm-package-path}} --target=bundler ./{{wasm-crate}} --scope caido

build-wasm-dev:
  wasm-pack build --out-name index --out-dir ../{{wasm-package-path}} --target=bundler ./{{wasm-crate}} --dev

wasm-tests:
  yarn replace-in-file "/\"module\":/" "\"type\": \"module\", \"main\":" "{{wasm-package-path}}/package.json" --isRegex
  yarn prettier --write "{{wasm-package-path}}/package.json"
  cd ./web-tests && yarn test

clippy:
  cargo clippy -p caido-convert
  cargo clippy -p convert --target wasm32-unknown-unknown

format:
  cargo +nightly fmt

format-check:
  cargo +nightly fmt --check

test:
  cargo test -p caido-convert
  just build-wasm-dev
  just wasm-tests

coverage:
  cargo llvm-cov -p caido-convert --html --open --ignore-filename-regex errors

lcov:
  cargo llvm-cov -p caido-convert --ignore-filename-regex errors --lcov --output-path lcov.info
