wasm-package-path := "./wasm-package"

default:
    @just --list

build-wasm-convert:
  wasm-pack build --out-dir ../wasm-package --target=bundler ./convert-wasm
  yarn replace-in-file "/\"module\":/" "\"type\": \"module\", \"main\":" "{{wasm-package-path}}/package.json" --isRegex
  yarn prettier --write "{{wasm-package-path}}/package.json"

wasm-tests: build-wasm-convert
  yarn workspace web test

test:
  cargo test -p convert
  just wasm-tests
