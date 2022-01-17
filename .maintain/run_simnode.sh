#!/bin/bash
#
# Runs simnode for runtimes whose files have changed.

#set -e # fail on any error

#shellcheck source=../common/lib.sh
. "$(dirname "${0}")/./common/lib.sh"

# shellcheck disable=SC2039
VERSIONS_FILES=(
  "picasso,picasso"
  "dali-chachacha,dali"
  # "composable,composable" # TODO: add simnode suppport for composable
)

/home/runner/.cargo/bin/rustup update nightly
/home/runner/.cargo/bin/rustup target add wasm32-unknown-unknown --toolchain nightly
/home/runner/.cargo/bin/cargo build --release -p simnode
YDATE=$(date -d yesterday +'%m-%d-%Y')

run_simnode() {
  CHAIN="$1"
  boldprint "Running simnode for $CHAIN"
  FILENAME=cl-1-$YDATE.zip
  GS_BUCKET="$CHAIN-data-store"
  sudo gsutil cp gs://$GS_BUCKET/"$FILENAME" .
  sudo unzip -o "$FILENAME" -d /tmp/db
  ./target/release/simnode --chain="$CHAIN" --base-path=/tmp/db/ --pruning=archive --execution=wasm
}

# shellcheck disable=SC2039
for i in "${VERSIONS_FILES[@]}"; do
  while IFS=',' read -r chain folder; do
    boldprint "check if the wasm sources changed for $chain"
    if has_runtime_changes "${LATEST_TAG_NAME}" "${GITHUB_REF_NAME}" "$folder"; then
      # shellcheck disable=SC2086
      run_simnode $chain
    fi
  done <<<"$i"
done
