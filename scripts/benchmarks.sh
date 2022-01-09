#!/bin/bash
# To run benchmarks `node` must be build with benchmarks enabled in release mode 
# `cargo build --release --features runtime-benchmarks`
steps=50
repeat=20
picassoOutput=./runtime/picasso/src/weights
picassoChain=picasso-dev
pallets=(
	oracle
	frame_system
	timestamp
	session
	balances
	indices
	membership
	treasury
	scheduler
	collective
	democracy
	collator_selection
	utility
	lending
	dutch_auction
)

for p in ${pallets[@]}
do
	./target/release/composable benchmark \
		--chain=$picassoChain \
		--execution=wasm \
		--wasm-execution=compiled \
		--pallet=$p  \
		--extrinsic='*' \
		--steps=$steps  \
		--repeat=$repeat \
		--raw  \
		--output=$picassoOutput

done
