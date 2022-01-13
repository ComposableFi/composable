#!/bin/bash
#
# check for any changes in the node/src/runtime, frame/* and primitives/sr_* trees. if
# there are any changes found, it should mark the PR breaksconsensus and
# "auto-fail" the PR if there isn't a change in the runtime/src/lib.rs file
# that alters the version.

set -e # fail on any error

#shellcheck source=../common/lib.sh
. "$(dirname "${0}")/./common/lib.sh"

declare -a VERSIONS_FILES=(
  "runtime/picasso/src/lib.rs,picasso,picasso"
  "runtime/dali/src/lib.rs,dali-chachacha,dali"
  "runtime/composable/src/lib.rs,composable,composable"
)

RELEASE_VERSION=$(git tag --sort=committerdate | grep -E '^[0-9]' | tail -1 )
COMMIT_SHA=$(git rev-parse --short=9 HEAD)
GITHUB_REF_NAME=$(git rev-parse --abbrev-ref HEAD)


boldprint () { printf "|\n| \033[1m%s\033[0m\n|\n" "${@}"; }
boldcat () { printf "|\n"; while read -r l; do printf "| \033[1m%s\033[0m\n" "${l}"; done; printf "|\n" ; }


boldprint "latest 10 commits of ${GITHUB_REF_NAME}"
git log --graph --oneline --decorate=short -n 10

boldprint "make sure the main branch and release tag are available in shallow clones"
git fetch --depth="${GIT_DEPTH:-100}" origin main
git fetch --depth="${GIT_DEPTH:-100}" origin releases
git tag -f releases FETCH_HEAD
git log -n1 releases

simnode_check () {
  VERSIONS_FILE="$1"
if has_runtime_changes origin/main "${GITHUB_REF_NAME}" $3 && check_runtime $VERSIONS_FILE $2
  boldprint "Checking for conditions to run simnode"
then
  boldprint "Running simnode for INtegration test OK"
fi
}


check_runtime() {
  VERSIONS_FILE="$1"
add_spec_version="$(git diff origin tags/releases ${GITHUB_REF_NAME} -- "${VERSIONS_FILE}" \
	| sed -n -r "s/^\+[[:space:]]+spec_version: +([0-9]+),$/\1/p")"
sub_spec_version="$(git diff tags/releases ${GITHUB_REF_NAME} -- "${VERSIONS_FILE}" \
	| sed -n -r "s/^\-[[:space:]]+spec_version: +([0-9]+),$/\1/p")"
if [ "${add_spec_version}" != "${sub_spec_version}" ]
then

	boldcat <<-EOT

		changes to the runtime sources and changes in the spec version.

		spec_version: ${sub_spec_version} -> ${add_spec_version}

	EOT
	return 0

else
	# check for impl_version updates: if only the impl versions changed, we assume
	# there is no consensus-critical logic that has changed.
	
	add_impl_version="$(git diff tags/releases ${GITHUB_REF_NAME} -- "${VERSIONS_FILE}" \
		| sed -n -r 's/^\+[[:space:]]+impl_version: +([0-9]+),$/\1/p')"
	sub_impl_version="$(git diff tags/releases ${GITHUB_REF_NAME} -- "${VERSIONS_FILE}" \
		| sed -n -r 's/^\-[[:space:]]+impl_version: +([0-9]+),$/\1/p')"


	# see if the impl version changed
	if [ "${add_impl_version}" != "${sub_impl_version}" ]
	then
		boldcat <<-EOT

		changes to the runtime sources and changes in the impl version.

		impl_version: ${sub_impl_version} -> ${add_impl_version}

		EOT
		return 0
	fi


	boldcat <<-EOT

	wasm source files changed but not the spec/impl version. If changes made do not alter logic,
	just bump 'impl_version'. If they do change logic, bump 'spec_version'.

	source file directories:
	- frame/*
	- runtime/$2/*

	versions file: ${VERSIONS_FILE}

	EOT
	return 1
fi
}

boldprint "check if the runtime changed and run simnode"
for i in "${VERSIONS_FILES[@]}"; do
  while IFS=',' read -r output chain folder; do
      echo "$chain"
      boldprint "check if the wasm sources changed"
      simnode_check $output $chain $folder
  done <<< "$i"
done

# dropped through. there's something wrong;  exit 1.
exit 1

# vim: noexpandtab
