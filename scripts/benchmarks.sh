#!/bin/bash

steps=5
repeat=2
picassoOutput=./runtime/src/weights/
picassoChain=dev
pallets=(
	pallet_oracle
)

for p in ${pallets[@]}
do
	./target/release/node-template benchmark \
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
