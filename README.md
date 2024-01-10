# wasmark

Bidirectional Markdown Parser implemented in WebAssembly

Supporting Markdown to HTML, and HTML to Markdown.

## How to develop

Currently, the code is really simple.
So, just fix the code and run the `wasm-pack` command to generate JS and WASM.

```sh
wasm-pack build --target bundler
```

Release build

```sh
wasm-pack build --target bundler --release

wasm-opt -Oz -o ./pkg/wasmark_bg_optimized.wasm ./pkg/wasmark_bg.wasm
rm ./pkg/wasmark_bg.wasm
mv ./pkg/wasmark_bg_optimized.wasm ./pkg/wasmark_bg.wasm # Use Optimized version
```

### Pre-Requisite

[wasm-pack](https://github.com/rustwasm/wasm-pack) and [binaryen](https://github.com/WebAssembly/binaryen) (for `wasm-opt`)

```sh
cargo install wasm-pack

brew install binaryen
```
