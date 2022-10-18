#!/usr/bin/env bash
set -e
set -o xtrace

cd code
cargo +nightly build --release -p wasm-optimizer
cargo +nightly build --release -p composable-runtime-wasm --target wasm32-unknown-unknown
cargo +nightly build --release -p picasso-runtime-wasm --target wasm32-unknown-unknown
cargo +nightly build --release -p dali-runtime-wasm --target wasm32-unknown-unknown
./target/release/wasm-optimizer --input ./target/wasm32-unknown-unknown/release/dali_runtime.wasm --output ./target/wasm32-unknown-unknown/release/dali_runtime.optimized.wasm
./target/release/wasm-optimizer --input ./target/wasm32-unknown-unknown/release/picasso_runtime.wasm --output ./target/wasm32-unknown-unknown/release/picasso_runtime.optimized.wasm
./target/release/wasm-optimizer --input ./target/wasm32-unknown-unknown/release/composable_runtime.wasm --output ./target/wasm32-unknown-unknown/release/composable_runtime.optimized.wasm
export DALI_RUNTIME=$(pwd)/target/wasm32-unknown-unknown/release/dali_runtime.optimized.wasm
export PICASSO_RUNTIME=$(pwd)/target/wasm32-unknown-unknown/release/picasso_runtime.optimized.wasm
export COMPOSABLE_RUNTIME=$(pwd)/target/wasm32-unknown-unknown/release/composable_runtime.optimized.wasm
cargo +nightly build -p composable --features=builtin-wasm --release -v
