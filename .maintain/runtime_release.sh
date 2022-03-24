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
  "runtime/dali/src/lib.rs,dali-rococo,dali"
  "runtime/composable/src/lib.rs,composable,composable"
)

build_runtime () {
  chain=$3
  # srtool for reproducible builds
  srtool build --package "$chain"-runtime --profile release --runtime-dir runtime/"$chain"
  # subwasm for runtime metadata
  printf "\n# %s Runtime\n\n" "$(echo $chain | awk '{ print toupper(substr($0, 1, 1)) substr($0, 2) }')" >> release.md
  INFO=$(subwasm info ./runtime/"$chain"/target/srtool/release/wbuild/"$chain"-runtime/"$chain"_runtime.compact.compressed.wasm)
  printf "\`\`\`\n%s\n\`\`\`\n" "$INFO" >> release.md
}

# Check which runtimes have changed and build them
for i in "${VERSIONS_FILES[@]}"; do
  while IFS=',' read -r output chain folder; do
    echo "check if the wasm sources changed for $chain"
    if has_runtime_changes "${PREV_TAG_OR_COMMIT}" "origin/${GITHUB_REF_NAME}" "$folder"
    then
      build_runtime $output $chain $folder
      echo ""$chain"_wasm=1" >> "$GITHUB_ENV"
    fi
  done <<< "$i"
done