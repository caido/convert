[![Tests](https://github.com/caido/convert/actions/workflows/validate.yml/badge.svg?branch=main)](https://github.com/caido/convert/actions/workflows/validate.yml?query=branch%3Amain)
[![codecov](https://codecov.io/gh/caido/convert/branch/main/graph/badge.svg?token=G3HA8K9IIX)](https://codecov.io/gh/caido/convert)
# 👋 Welcome

Welcome to caido convert packages! This repo contains the code we use to encode, hash, zip and much more.
The core logic is written in caido/convert crate. The javascript binding are provided by caido-convert and publish as an npm package.  

## What you will need to build it 🛠️
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)
- [rust](https://www.rust-lang.org/tools/install)
- wasm-opt `cargo install wasm-opt`
- just `cargo install just`
- llvm-cov `cargo install cargo-llvm-cov`
