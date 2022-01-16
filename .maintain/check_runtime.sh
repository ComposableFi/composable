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

boldprint "latest 10 commits of ${GITHUB_REF_NAME}"
git log --graph --oneline --decorate=short -n 10

boldprint "make sure the main branch and release tag are available in shallow clones"
git fetch --depth="${GIT_DEPTH:-100}" origin main
git fetch --depth="${GIT_DEPTH:-100}" origin "${RELEASE_VERSION}"
git tag -f "${RELEASE_VERSION}" FETCH_HEAD
git log -n1 "${RELEASE_VERSION}"

simnode_check () {
  VERSIONS_FILE="$1"
if has_runtime_changes "${RELEASE_VERSION}" "${GITHUB_REF_NAME}" "$3" && check_runtime "$VERSIONS_FILE" "$2"
then
   boldprint "Checking for conditions to run simnode"
	echo "RUNTIME_CHECK=1" >> "$GITHUB_ENV"
fi
}



boldprint "check if the runtime changed and run simnode"
for i in "${VERSIONS_FILES[@]}"; do
  while IFS=',' read -r output chain folder; do
      boldprint "check if the wasm sources changed for $chain"
      simnode_check $output $chain $folder
  done <<< "$i"
done

# dropped through. there's something wrong;  exit 1.
exit 0

# vim: noexpandtab
