#!/bin/sh
rm -rf ./pkg
wasm-pack build --target bundler --release

wasm-opt -Oz -o ./pkg/wasma_bg_optimized.wasm ./pkg/wasma_bg.wasm
rm ./pkg/wasma_bg.wasm
mv ./pkg/wasma_bg_optimized.wasm ./pkg/wasma_bg.wasm # Use Optimized version

rm ./pkg/package.json ./pkg/README.md ./pkg/.gitignore