#!/bin/bash
#
# Runs benchmarks for runtimes whose files have changed.

set -e # fail on any error

#shellcheck source=../common/lib.sh
. "$(dirname "${0}")/./common/lib.sh"

VERSIONS_FILES=(
  "runtime/picasso/src/weights,picasso-dev,picasso"
  "runtime/dali/src/weights,dali-dev,dali"
  "runtime/composable/src/weights,composable-dev,composable"
)

steps=$1
repeat=$2

/home/runner/.cargo/bin/rustup install nightly
/home/runner/.cargo/bin/rustup target add wasm32-unknown-unknown --toolchain nightly
/home/runner/.cargo/bin/cargo build --release -p composable --features=runtime-benchmarks

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
