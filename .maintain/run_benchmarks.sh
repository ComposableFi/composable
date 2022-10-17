#!/bin/bash
#
# Runs benchmarks for runtimes whose files have changed.

set -e # fail on any error

#shellcheck source=../common/lib.sh
. "$(dirname "${0}")/./common/lib.sh"

VERSIONS_FILES=(
  "parachain/runtime/picasso/src/weights,picasso-dev,picasso"
  "parachain/runtime/dali/src/weights,dali-dev,dali"
  "parachain/runtime/composable/src/weights,composable-dev,composable"
)

steps=${1:-1}
repeat=${2:-1}

# NOTE: decide prio and responsible for migration to nix after https://github.com/ComposableFi/composable/issues/1426
cd code
cargo +nightly build --release -p wasm-optimizer
cargo +nightly build --release -p composable-runtime-wasm --target wasm32-unknown-unknown --features=runtime-benchmarks
cargo +nightly build --release -p picasso-runtime-wasm --target wasm32-unknown-unknown --features=runtime-benchmarks
cargo +nightly build --release -p dali-runtime-wasm --target wasm32-unknown-unknown --features=runtime-benchmarks
./target/release/wasm-optimizer --input ./target/wasm32-unknown-unknown/release/dali_runtime.wasm --output ./target/wasm32-unknown-unknown/release/dali_runtime.optimized.wasm
./target/release/wasm-optimizer --input ./target/wasm32-unknown-unknown/release/picasso_runtime.wasm --output ./target/wasm32-unknown-unknown/release/picasso_runtime.optimized.wasm
./target/release/wasm-optimizer --input ./target/wasm32-unknown-unknown/release/composable_runtime.wasm --output ./target/wasm32-unknown-unknown/release/composable_runtime.optimized.wasm
export DALI_RUNTIME=$(realpath ./target/wasm32-unknown-unknown/release/dali_runtime.optimized.wasm)
export PICASSO_RUNTIME=$(realpath ./target/wasm32-unknown-unknown/release/picasso_runtime.optimized.wasm)
export COMPOSABLE_RUNTIME=$(realpath ./target/wasm32-unknown-unknown/release/composable_runtime.optimized.wasm)

# TODO: use nix
cargo build --release --package composable --features=runtime-benchmarks --features=builtin-wasm

run_benchmarks() {
  OUTPUT=$1
  CHAIN=$2
  FOLDER=$3
  # shellcheck disable=SC2068
  echo "Running benchmarks for $CHAIN"
  # shellcheck disable=SC2068
  ./target/release/composable benchmark pallet \
    --chain="$CHAIN" \
    --execution=wasm \
    --wasm-execution=compiled \
    --wasm-instantiation-strategy=legacy-instance-reuse \
    --pallet="*" \
    --extrinsic='*' \
    --steps=$steps \
    --repeat=$repeat \
    --output="$OUTPUT" \
    --log error
  # ToDO: Setup gpg signing and create a bot account for pushing
}

for i in "${VERSIONS_FILES[@]}"; do
  while IFS=',' read -r output chain folder; do
     if has_runtime_changes "origin/${BASE_BRANCH}" "origin/${GITHUB_BRANCH_NAME}" "$folder"; then
       run_benchmarks $output $chain $folder
     fi
  done <<<"$i"
done
