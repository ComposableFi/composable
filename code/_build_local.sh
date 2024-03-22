#!/bin/bash

wasm-optimizer() {
    wasm-opt $1 -o $2 -Os --strip-dwarf --debuginfo --mvp-features
    subwasm compress $2 $2
}

cargo build -p composable-runtime-wasm --target wasm32-unknown-unknown --features testnet -r || exit 1
cargo build -p picasso-runtime-wasm    --target wasm32-unknown-unknown --features testnet -r || exit 1

wasm-optimizer ./target/wasm32-unknown-unknown/release/picasso_runtime.wasm    ./target/wasm32-unknown-unknown/release/picasso_runtime.optimized.wasm
wasm-optimizer ./target/wasm32-unknown-unknown/release/composable_runtime.wasm ./target/wasm32-unknown-unknown/release/composable_runtime.optimized.wasm

# export PICASSO_RUNTIME=$(realpath ./target/wasm32-unknown-unknown/release/picasso_runtime.optimized.wasm)
# export COMPOSABLE_RUNTIME=$(realpath ./target/wasm32-unknown-unknown/debug/composable_runtime.optimized.wasm)

# cargo build --features builtin-wasm --bin composable -r || exit 1
cargo build -p composable-runtime || exit 1
# exit 1