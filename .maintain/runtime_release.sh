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

LATEST_TAG_NAME=$(get_latest_release ComposableFi/composable)
GITHUB_REF_NAME=$(git rev-parse --abbrev-ref HEAD)

/home/runner/.cargo/bin/cargo install --git https://github.com/chevdor/srtool-cli
/home/runner/.cargo/bin/cargo install --locked --git https://github.com/chevdor/subwasm --tag v0.16.1

boldprint () { printf "|\n| \033[1m%s\033[0m\n|\n" "${@}"; }
boldcat () { printf "|\n"; while read -r l; do printf "| \033[1m%s\033[0m\n" "${l}"; done; printf "|\n" ; }



boldprint "check if the runtime changed and run simnode"
for i in "${VERSIONS_FILES[@]}"; do
  while IFS=',' read -r output chain folder; do
    boldprint "check if the wasm sources changed for $chain"
    if has_runtime_changes "${LATEST_TAG_NAME}" "${GITHUB_REF_NAME}" "$folder"
    then
      build_runtime $output $chain $folder
    fi
  done <<< "$i"
done

build_runtime () {
  chain=$3
  /home/runner/.cargo/bin/srtool build --package "$chain"-runtime --profile release --runtime-dir ./runtime/"$chain"
  /home/runner/.cargo/bin/subwasm info ./runtime/"$chain"/target/srtool/release/wbuild/"$chain"-runtime/"$chain"_runtime.compact.wasm >> "$chain"-release.md
}