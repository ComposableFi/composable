#!/bin/bash

steps=5
repeat=2
picassoOutput=./runtime/picasso/src/weights
picassoChain=picasso-dev
pallets=(
	oracle
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
