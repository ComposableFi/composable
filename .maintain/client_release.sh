#!/bin/bash
#
# check for any changes in the runtime/ and frame/*. if
# there are any changes found, it should mark the PR breaksconsensus and
# "auto-fail" the PR if there isn't a change in the runtime/src/lib.rs file
# that alters the version.

set -e # fail on any error

#shellcheck source=../common/lib.sh
. "$(dirname "${0}")/./common/lib.sh"

RELEASE_VERSION=$(git tag --sort=committerdate | grep -E '^v[0-9]' | tail -1)
# Because this script runs when a tag has been published, the previous tag is the
# last two tags
PREV_TAG=$(gh release list -L=2 | sed -n '2 p' | awk '{print $(NF-1)}')

if has_client_changes "${PREV_TAG}" "${GITHUB_REF_NAME}"; then
  boldprint "Building new client binaries"
  cargo build --release -p composable
  tar -czvf composable-"${RELEASE_VERSION}".tar.gz target/release/composable
  tar -czvf picasso_runtime.compact.wasm-"${RELEASE_VERSION}".tar.gz runtime/picasso/target/srtool/release/wbuild/picasso-runtime/picasso_runtime.compact.wasm
  gsutil cp *.tar.gz gs://composable-binaries/community-releases/"${RELEASE_VERSION}"/
fi
