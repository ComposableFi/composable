#!/bin/bash
#
# check for any changes in the runtime/ and frame/*. if
# there are any changes found, it should mark the PR breaksconsensus and
# "auto-fail" the PR if there isn't a change in the runtime/src/lib.rs file
# that alters the version.

set -e # fail on any error

#shellcheck source=../common/lib.sh
. "$(dirname "${0}")/./common/lib.sh"

# shellcheck disable=SC2039
VERSIONS_FILES=(
  "runtime/picasso/src/lib.rs,picasso,picasso"
  "runtime/dali/src/lib.rs,dali-chachacha,dali"
  "runtime/composable/src/lib.rs,composable,composable"
)
# Because this script runs when a tag has been published, the previous tag is the
# last two tags
PREV_TAG=$(gh release list -L=2 | sed -n '2 p' | awk '{print $(NF-1)}')
CURRENT_TAG=$(gh release list -L=1 | sed -n '1 p' | awk '{print $(NF-1)}')

# Install the neccessary tools needed for building
cargo install --git https://github.com/chevdor/srtool-cli
cargo install --locked --git https://github.com/chevdor/subwasm --tag v0.16.1

build_runtime() {
  chain=$3
  # srtool for reproducible builds
  srtool build --package "$chain"-runtime --profile release --runtime-dir runtime/"$chain"
  # subwasm for runtime metadata
  echo "# $chain Runtime " >>release.md
  subwasm info ./runtime/"$chain"/target/srtool/release/wbuild/"$chain"-runtime/"$chain"_runtime.compact.wasm >>release.md
}

# Check which runtimes have changed and build them
for i in "${VERSIONS_FILES[@]}"; do
  while IFS=',' read -r output chain folder; do
    echo "check if the wasm sources changed for $chain"
    if has_runtime_changes "${PREV_TAG}" "${GITHUB_REF_NAME}" "$folder"; then
      build_runtime $output $chain $folder
      CHANGES=gh view release tag $CURRENT_TAG
      echo $CHANGES | sed '1,/--/  d' >>release.md
      echo "$chain-wasm=1" >>"$GITHUB_ENV"
    fi
  done <<<"$i"
done
