# Wasm + TypeScript + ESM in Node.js, Jest and Next.js

This repo contains a web app that depends on an NPM package with [Wasm](https://webassembly.org/), TypeScript and native ESModules.
Wasm code is successfully used in a Node.js script, a Jest test and a Next.js app within a page and an API route.

The demo was created while working on [blockprotocol/blockprotocol](https://github.com/blockprotocol/blockprotocol) and [hashintel/hash](https://github.com/hashintel/hash).

## Prerequisites

- [Node.js](https://nodejs.org/en/) v16 or higher
- [Yarn](https://yarnpkg.com/)
- [Rust](https://www.rust-lang.org/tools/install) and [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/) (unless you want to skip steps 2 and 3 of the demo)

## Demo steps

1.  Clone the repo and install dependencies from the root folder:

    ```sh
    yarn install
    ```

1.  _(can be skipped)_ Generate `wasm-package` from `wasm-crate`:

    ```sh
    cd wasm-crate
    wasm-pack build --out-dir ../wasm-package --target=bundler
    cd ..
    ```

1.  _(can be skipped)_ Tweak `wasm-package`:

    ```sh
    ## Enable native ESM in package.json
    yarn replace-in-file "/\"module\":/" "\"type\": \"module\", \"main\":" wasm-package/package.json --isRegex
    yarn prettier --write wasm-package/package.json

    ## Remove autogenerated .gitignore
    rm wasm-package/.gitignore
    ```

    This step won’t be necessary when [rustwasm/wasm-pack#1061](https://github.com/rustwasm/wasm-pack/pull/1061) is merged and released.

1.  🎉 Run a Node.js script that uses the `wasm-package`:

    ```sh
    yarn workspace web-app exe scripts/wasm-package-answer.ts
    ```

1.  🎉 Run unit tests referring to the `wasm-package`:

    ```sh
    yarn workspace web-app test
    ```

    Needs Jest `>=29.3.0`, see [facebook/jest#13505](https://github.com/facebook/jest/pull/13505).

1.  🎉 Run Next.js dev server and check if `wasm-package` works there too:

    ```sh
    yarn workspace web dev
    ```

    Open [localhost:3000](http://localhost:3000) and [localhost:3000/api/wasm-package-answer](http://localhost:3000/api/wasm-package-answer) to see the result.

    ⚠️ Note the [workaround](https://github.com/vercel/next.js/issues/29362#issuecomment-971377869) in [web-app/next.config.js](web-app/next.config.js).

1.  🎉 Build and run production Next.js app:

    ```sh
    yarn workspace web-app build
    yarn workspace web-app start
    ```
