#!/bin/bash
#
# check for any changes in the runtime/ and frame/*. if
# there are any changes found, it should mark the PR breaksconsensus and
# "auto-fail" the PR if there isn't a change in the runtime/src/lib.rs file
# that alters the version.

#set -e # fail on any error

#shellcheck source=../common/lib.sh
. "$(dirname "${0}")/./common/lib.sh"

LATEST_TAG_NAME=$(get_latest_release ComposableFi/composable)
GITHUB_REF_NAME=$(git rev-parse --abbrev-ref HEAD)


boldprint () { printf "|\n| \033[1m%s\033[0m\n|\n" "${@}"; }
boldcat () { printf "|\n"; while read -r l; do printf "| \033[1m%s\033[0m\n" "${l}"; done; printf "|\n" ; }


if has_client_changes "${LATEST_TAG_NAME}" "${GITHUB_REF_NAME}"
then
  cargo build --release -p composable
  tar -czvf composable-${{ RELEASE_VERSION }}.tar.gz target/release/composable
  tar -czvf picasso_runtime.compact.wasm-${RELEASE_VERSION}.tar.gz runtime/picasso/target/srtool/release/wbuild/picasso-runtime/picasso_runtime.compact.wasm
  gsutil cp *.tar.gz gs://composable-binaries/community-releases/${{ RELEASE_VERSION }}/
fi