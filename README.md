# wasma

Bidirectional Markdown Parser implemented in WebAssembly

Supporting Markdown to HTML, and HTML to Markdown.

(In fact, I wanted to name `wasmark`, but already taken in npm so `wasma`...)

## How to develop

Currently, the code is really simple.
So, just fix the code and run the `wasm-pack` command to generate JS and WASM.

```sh
wasm-pack build --target bundler
```

Release build => See [./scripts/release-build.sh](./scripts/release-build.sh)

### Pre-Requisite

[wasm-pack](https://github.com/rustwasm/wasm-pack) and [binaryen](https://github.com/WebAssembly/binaryen) (for `wasm-opt`)

```sh
cargo install wasm-pack

brew install binaryen
```
