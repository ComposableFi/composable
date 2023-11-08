#!/bin/sh

set -eu

target=wasm32-unknown-unknown

RUSTFLAGS='-C link-arg=-s'
export RUSTFLAGS

for pkg in cw-xc-gateway cw-xc-interpreter; do
	cargo build -p "$pkg" --profile cosmwasm-contracts --target "$target"
done
