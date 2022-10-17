#!/bin/bash
#
# Runs simnode for runtimes whose files have changed.

set -e # fail on any error

#shellcheck source=../common/lib.sh
. "$(dirname "${0}")/./common/lib.sh"

# shellcheck disable=SC2039
VERSIONS_FILES=(
  "picasso,picasso"
  "dali-rococo,dali"
  "composable,composable"
)

/home/runner/.cargo/bin/rustup update nightly
/home/runner/.cargo/bin/rustup target add wasm32-unknown-unknown --toolchain nightly
/home/runner/.cargo/bin/cargo build --release -p simnode-tests
sudo chown -R runner:runner target/release/simnode-tests && sudo chmod +x target/release/simnode-tests
sudo mkdir -p /tmp/db && sudo chown -R runner:runner /tmp/db
YDATE=$(date -d yesterday +'%m-%d-%Y')

run_simnode() {
  CHAIN="$1"
  echo "Running simnode for $CHAIN "
  FILENAME=$YDATE.zip
  GS_BUCKET="$CHAIN-data-store"
  sudo gsutil cp gs://$GS_BUCKET/"$FILENAME" .
  sudo unzip -o "$FILENAME" -d /tmp/db
  sudo ./target/release/simnode-tests --chain="$CHAIN" --base-path=/tmp/db/var/lib/composable-data/ --pruning=archive --execution=wasm
}

# shellcheck disable=SC2039
for i in "${VERSIONS_FILES[@]}"; do
  while IFS=',' read -r chain folder; do
    echo "check if the wasm sources changed for $chain"
    if has_runtime_changes "origin/${BASE_BRANCH}" "origin/${GITHUB_BRANCH_NAME}" "$folder"; then
      # shellcheck disable=SC2086
      run_simnode $chain
    fi
  done <<<"$i"
done
