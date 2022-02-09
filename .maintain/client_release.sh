#!/bin/bash
#
# check for any changes in the runtime/ and frame/*. if
# there are any changes found, it should mark the PR breaksconsensus and
# "auto-fail" the PR if there isn't a change in the runtime/src/lib.rs file
# that alters the version.

set -e # fail on any error

#shellcheck source=../common/lib.sh
. "$(dirname "${0}")/./common/lib.sh"

HAS_CLIENT_CHANGES=$(has_client_changes "${PREV_TAG_OR_COMMIT}" "${GITHUB_REF_NAME}")

if [ "$HAS_CLIENT_CHANGES" ] || [ "$FORCE_CLIENT_BUILD" == 1 ]; then
  boldprint "Building new client binaries"
  cargo build --release -p composable
  tar -czvf composable-"${RELEASE_VERSION}".tar.gz target/release/composable
  gsutil cp *.tar.gz gs://composable-binaries/community-releases/"${RELEASE_VERSION}"/
  echo "client_release=1" >> "$GITHUB_ENV"
fi
