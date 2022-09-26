#!/bin/bash
#set -e

cd code && \
cargo build --release -p wasm-optimizer && \
cargo build --release -p composable-runtime-wasm --target wasm32-unknown-unknown && \
cargo build --release -p picasso-runtime-wasm --target wasm32-unknown-unknown && \
cargo build --release -p dali-runtime-wasm --target wasm32-unknown-unknown && \
./target/release/wasm-optimizer --input ./target/wasm32-unknown-unknown/release/dali_runtime.wasm --output ./target/wasm32-unknown-unknown/release/dali_runtime.optimized.wasm && \
./target/release/wasm-optimizer --input ./target/wasm32-unknown-unknown/release/picasso_runtime.wasm --output ./target/wasm32-unknown-unknown/release/picasso_runtime.optimized.wasm && \
./target/release/wasm-optimizer --input ./target/wasm32-unknown-unknown/release/composable_runtime.wasm --output ./target/wasm32-unknown-unknown/release/composable_runtime.optimized.wasm && \
export DALI_RUNTIME=$(realpath ./target/wasm32-unknown-unknown/release/dali_runtime.optimized.wasm)
export PICASSO_RUNTIME=$(realpath ./target/wasm32-unknown-unknown/release/picasso_runtime.optimized.wasm)
export COMPOSABLE_RUNTIME=$(realpath ./target/wasm32-unknown-unknown/release/composable_runtime.optimized.wasm)
cargo build --release --package composable --features=builtin-wasm
