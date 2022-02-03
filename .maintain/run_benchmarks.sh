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

steps=50
repeat=20

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
  for p in ${pallets[@]}; do
    ./target/release/composable benchmark \
      --chain="$CHAIN" \
      --execution=wasm \
      --wasm-execution=compiled \
      --pallet="$p" \
      --extrinsic='*' \
      --steps=$steps \
      --repeat=$repeat \
      --raw \
      --output="$OUTPUT"
  done
  git config --global user.email "haroldsphinx@gmail.com"
  git config --global user.name "haroldsphinx"
  USERNAME=$(gcloud secrets versions access latest --secret=github-api-username)
  PASSWORD=$(gcloud secrets versions access latest --secret=github-api-token)
  git remote set-url origin https://"$USERNAME":"$PASSWORD"@github.com/ComposableFi/composable.git
  git pull origin "$GITHUB_BRANCH_NAME"
  git add runtime/"$FOLDER"
  git commit -m "Updates weights for $CHAIN"
  git push origin "$GITHUB_BRANCH_NAME"
  # ToDO: Setup gpg signing and create a bot account for pushing
}

for i in "${VERSIONS_FILES[@]}"; do
  while IFS=',' read -r output chain folder; do
    if has_runtime_changes "${BASE_BRANCH}" "${GITHUB_BRANCH_NAME}" "$folder"; then
      run_benchmarks $output $chain $folder
    fi
  done <<<"$i"
done
