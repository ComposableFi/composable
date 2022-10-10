#!/bin/bash
#
# check for any changes in the runtime/ and frame/*. if
# there are any changes found, it should mark the PR breaksconsensus and
# "auto-fail" the PR if there isn't a change in the runtime/src/lib.rs file
# that alters the version.

# TODO: it was disabled on old pipeline, rewive it based on source/git src filters of nix
# FIXME: actually broken: https://github.com/ComposableFi/composable/runs/5570301249?check_suite_focus=true
# - name: Check for runtime changes
#   env:
#     BASE_BRANCH: ${{ github.event.pull_request.base.ref }}
#     GITHUB_BRANCH_NAME: ${{ steps.branch-name.outputs.current_branch }}
#   id: check_runtime
#   run: .maintain/check_runtime.sh

set -e # fail on any error

#shellcheck source=../common/lib.sh
. "$(dirname "${0}")/./common/lib.sh"

# shellcheck disable=SC2039
VERSIONS_FILES=(
  "code/parachain/runtime/picasso/src/lib.rs,picasso,picasso"
  "code/parachain/runtime/dali/src/lib.rs,dali-rococo,dali"
  "code/parachain/runtime/composable/src/lib.rs,composable,composable"
)

echo "make sure the main branch and release tag are available in shallow clones"
git fetch --depth="${GIT_DEPTH:-100}" origin "${BASE_BRANCH}"

simnode_check() {
  VERSIONS_FILE="$1"
  if has_runtime_changes "origin/${BASE_BRANCH}" "origin/${GITHUB_BRANCH_NAME}" "$2" && check_runtime "$VERSIONS_FILE" "$2"; then
    echo "Wasm sources have changed for $3"
    echo "RUNTIME_CHECK=1" >> $GITHUB_ENV
  fi
}

for i in "${VERSIONS_FILES[@]}"; do
  while IFS=',' read -r output chain folder; do
    boldprint "Check if the wasm sources changed for $chain"
    simnode_check $output $folder $chain
  done <<<"$i"
done

# dropped through. there's something wrong;  exit 1.
exit 0

# vim: noexpandtab
