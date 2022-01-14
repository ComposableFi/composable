#!/bin/bash
#
# Runs benchmarks for runtimes whose files have changed.

#set -e # fail on any error

#shellcheck source=../common/lib.sh
. "$(dirname "${0}")/./common/lib.sh"

VERSIONS_FILES=(
  "runtime/picasso/src/weights,picasso,picasso"
  "runtime/dali/src/weights,dali-chachacha,dali"
  "runtime/composable/src/weights,composable,composable"
)

LATEST_TAG_NAME=$(git tag --sort=committerdate | grep -E '^v[0-9]' | tail -1 )
GITHUB_REF_NAME=$(git rev-parse --abbrev-ref HEAD)

steps=50
repeat=20


/home/runner/.cargo/bin/rustup install nightly
/home/runner/.cargo/bin/rustup target add wasm32-unknown-unknown --toolchain nightly
/home/runner/.cargo/bin/cargo build --release -p composable --features=runtime-benchmarks


run_benchmarks () {
    OUTPUT=$1
    CHAIN=$2
    FOLDER=$3
    if has_runtime_changes release/${LATEST_TAG_NAME} "${GITHUB_REF_NAME}" $FOLDER
then
    # shellcheck disable=SC2068
    boldprint "Running benchmarks for $CHAIN"
    for p in ${pallets[@]}; do
	    ./target/release/composable benchmark \
		    --chain="$CHAIN" \
		    --execution=wasm \
		    --wasm-execution=compiled \
		    --pallet="$p"  \
		    --extrinsic='*' \
		    --steps=$steps  \
		    --repeat=$repeat \
		    --raw  \
		    --output="$OUTPUT"
    done
git add runtime
git commit -m "Updates weights"
git push origin $GITHUB_REF_NAME # does this work?
fi
}

boldprint "Running benchmarks"
for i in "${VERSIONS_FILES[@]}"; do
  while IFS=',' read -r output chain folder; do
      run_benchmarks $output $chain $folder
  done <<< "$i"
done