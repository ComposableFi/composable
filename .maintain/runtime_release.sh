#!/bin/bash
#
# check for any changes in the runtime/ and frame/*. if
# there are any changes found, it should mark the PR breaksconsensus and
# "auto-fail" the PR if there isn't a change in the runtime/src/lib.rs file
# that alters the version.

#set -e # fail on any error

#shellcheck source=../common/lib.sh
. "$(dirname "${0}")/./common/lib.sh"

# shellcheck disable=SC2039
VERSIONS_FILES=(
  "runtime/picasso/src/lib.rs,picasso,picasso"
  "runtime/dali/src/lib.rs,dali-chachacha,dali"
   "runtime/composable/src/lib.rs,composable,composable"
)

# Install the neccessary tools needed for building
cargo install --git https://github.com/chevdor/srtool-cli
cargo install --locked --git https://github.com/chevdor/subwasm --tag v0.16.1


build_runtime () {
  chain=$3
  # srtool for reproducible builds
  srtool build --package "$chain"-runtime --profile release --runtime-dir ./runtime/"$chain"
  # subwasm for runtime metadata
  echo "# $chain Runtime " >> release.md
  subwasm info ./runtime/"$chain"/target/srtool/release/wbuild/"$chain"-runtime/"$chain"_runtime.compact.wasm >> release.md
}

# Check which runtimes have changed and build them
for i in "${VERSIONS_FILES[@]}"; do
  while IFS=',' read -r output chain folder; do
    boldprint "check if the wasm sources changed for $chain"
    if has_runtime_changes "${LATEST_TAG_NAME}" "${GITHUB_REF_NAME}" "$folder"
    then
      build_runtime $output $chain $folder
      echo "$chain-wasm"
    fi
  done <<< "$i"
done

