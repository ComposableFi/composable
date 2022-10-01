#!/bin/bash

GITHUB_REF_NAME=$(git rev-parse --abbrev-ref HEAD)

get_latest_release() {
  curl --silent "https://api.github.com/repos/$1/releases/latest" | # Get latest release from GitHub api
    grep '"tag_name":' |                                            # Get tag line
    sed -E 's/.*"([^"]+)".*/\1/'                                    # Pluck JSON value
}

boldprint() { printf "|\n| \033[1m%s\033[0m\n|\n" "${@}"; }
boldcat() {
  printf "|\n"
  while read -r l; do printf "| \033[1m%s\033[0m\n" "${l}"; done
  printf "|\n"
}

LATEST_TAG_NAME=$(get_latest_release ComposableFi/composable)
#LATEST_TAG_NAME=$(gh release list -L=5 | sed -n '5 p' | awk '{print $(NF-1)}')
boldprint $LATEST_TAG_NAME
git fetch origin tag "${LATEST_TAG_NAME}" --no-tags

# We want to get the tag of the most reccent release.
# Used by client/runtime_release.sh
PREV_TAG_OR_COMMIT=$(git tag --sort=committerdate | grep -E '^v[0-9]' | tail -1)
# This is a special case where a draft release is already in progress.
# which also means that this is either a runtime_release/client_release
# workflow which runs on push. Lets diff with the last commit in the base branch then.
if [[ $PREV_TAG_OR_COMMIT == *"untagged"* ]]; then
    PREV_TAG_OR_COMMIT=$(git log -n 1 --skip 1 --pretty=format:"%H")
fi

# Check for runtime changes between two commits. This is defined as any changes
# to code/parachain/runtime/, code/parachain/frame/
has_runtime_changes() {
  from=$1
  to=$2
  echo "diffing $from & $to"
  if git diff --name-only "${from}...${to}" |
    grep -q -e '^code/parachain/frame/' -e "^code/parachain/runtime/$3/"; then
    return 0
  else
    return 1
  fi
}

# Check for client changes between two commits. This is defined as any changes
# to node/, src/
has_client_changes() {
  from=$1
  to=$2
  if git diff --name-only "${from}...${to}" |
    grep -q -e '^node/' -e "^src/"; then
    return 0
  else
    return 1
  fi
}

# checks if the spec/impl version has increased
check_runtime() {
  VERSIONS_FILE="$1"
  add_spec_version="$(git diff "${LATEST_TAG_NAME}" "origin/${GITHUB_BRANCH_NAME}" -- "${VERSIONS_FILE}" |
    sed -n -r "s/^\+[[:space:]]+spec_version: +([0-9]+),$/\1/p")"
  sub_spec_version="$(git diff "${LATEST_TAG_NAME}" "origin/${GITHUB_BRANCH_NAME}" -- "${VERSIONS_FILE}" |
    sed -n -r "s/^\-[[:space:]]+spec_version: +([0-9]+),$/\1/p")"
  if [ "${add_spec_version}" != "${sub_spec_version}" ]; then

    boldcat <<-EOT

		changes to the runtime sources and changes in the spec version.

		spec_version: ${sub_spec_version} -> ${add_spec_version}

	EOT
    return 0

  else
    # check for impl_version updates: if only the impl versions changed, we assume
    # there is no consensus-critical logic that has changed.

    add_impl_version="$(git diff "${LATEST_TAG_NAME}" "origin/${GITHUB_BRANCH_NAME}" -- "${VERSIONS_FILE}" |
      sed -n -r 's/^\+[[:space:]]+impl_version: +([0-9]+),$/\1/p')"
    sub_impl_version="$(git diff "${LATEST_TAG_NAME}" "origin/${GITHUB_BRANCH_NAME}" -- "${VERSIONS_FILE}" |
      sed -n -r 's/^\-[[:space:]]+impl_version: +([0-9]+),$/\1/p')"

    # see if the impl version changed
    if [ "${add_impl_version}" != "${sub_impl_version}" ]; then
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
	- code/parachain/frame/*
	- code/parachain/runtime/$2/*

	versions file: ${VERSIONS_FILE}

	EOT
    exit 1 # Exit because user needs to bump spec version
  fi
}
