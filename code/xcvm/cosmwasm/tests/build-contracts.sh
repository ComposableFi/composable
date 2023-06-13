#!/bin/sh

RUSTFLAGS='-C link-arg=-s'
export RUSTFLAGS
for pkg in cw-xc-asset-registry cw-xc-gateway cw-xc-interpreter; do
	cargo build -p "$pkg" \
	      --profile cosmwasm-contracts \
	      --target wasm32-unknown-unknown
done
