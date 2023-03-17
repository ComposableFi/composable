#!/usr/bin/env bash
set -e

# Default binary version
BINARY_VERSION=v1.10002
while getopts b: flag
do
    case "${flag}" in
        b) BINARY_VERSION=v${OPTARG};;
    esac
done
echo "Binary: $BINARY_VERSION";

CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
CURRENT_DIRECTORY=$(pwd)

BINARY=data/binary
WASM=data/runtime.wasm

cargo install wasm-opt --locked
cargo install --locked --git https://github.com/chevdor/subwasm --tag v0.16.1

cd ../../../code

git checkout release-$BINARY_VERSION > /dev/null 2>&1
cargo +nightly build --release -p composable-runtime-wasm --target wasm32-unknown-unknown
cargo +nightly build --release -p picasso-runtime-wasm --target wasm32-unknown-unknown
cargo +nightly build --release -p dali-runtime-wasm --target wasm32-unknown-unknown

wasm-optimizer() {
    wasm-opt $1 -o $2 -Os --strip-dwarf --debuginfo --mvp-features
    subwasm compress $2 $2
}

wasm-optimizer ./target/wasm32-unknown-unknown/release/dali_runtime.wasm ./target/wasm32-unknown-unknown/release/dali_runtime.optimized.wasm
wasm-optimizer ./target/wasm32-unknown-unknown/release/picasso_runtime.wasm ./target/wasm32-unknown-unknown/release/picasso_runtime.optimized.wasm
wasm-optimizer ./target/wasm32-unknown-unknown/release/composable_runtime.wasm ./target/wasm32-unknown-unknown/release/composable_runtime.optimized.wasm

export DALI_RUNTIME=$(realpath ./target/wasm32-unknown-unknown/release/dali_runtime.optimized.wasm) && \
export PICASSO_RUNTIME=$(realpath ./target/wasm32-unknown-unknown/release/picasso_runtime.optimized.wasm) && \
export COMPOSABLE_RUNTIME=$(realpath ./target/wasm32-unknown-unknown/release/composable_runtime.optimized.wasm) && \
cargo build --release --package composable --features=builtin-wasm

cp ./target/release/composable $CURRENT_DIRECTORY/$BINARY
cp ./target/wasm32-unknown-unknown/release/picasso_runtime.optimized.wasm $CURRENT_DIRECTORY/$WASM

git checkout $CURRENT_BRANCH > /dev/null 2>&1

echo "Binary and WASM successfully built!"
