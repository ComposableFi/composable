#!/bin/bash
# To run benchmarks `node` must be build with benchmarks enabled in release mode 
# `cargo build --release --features runtime-benchmarks`

chains=(
  "./runtime/composable/src/weights,composable-dev"
  "./runtime/picasso/src/weights,picasso-dev"
  "./runtime/dali/src/weights,dali-dev"
)

steps=50
repeat=20


# shellcheck disable=SC2068
for i in ${chains[@]}; do
  while IFS=',' read -r output chain; do
    # shellcheck disable=SC2068
	    ./target/release/composable benchmark \
		    --chain="$chain" \
		    --execution=wasm \
		    --wasm-execution=compiled \
		    --pallet="*"  \
		    --extrinsic='pallet_lending' \
		    --steps=$steps  \
		    --repeat=$repeat \
		    --raw  \
		    --output="$output"
  done <<< "$i"
done
