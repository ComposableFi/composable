#!/bin/bash

api_base="https://api.github.com/repos"
GITHUB_REF_NAME=$(git rev-parse HEAD)

# Function to take 2 git tags/commits and get any lines from commit messages
# that contain something that looks like a PR reference: e.g., (#1234)
sanitised_git_logs() {
  git --no-pager log --pretty=format:"%s" "$1...$2" |
    # Only find messages referencing a PR
    grep -E '\(#[0-9]+\)' |
    # Strip any asterisks
    sed 's/^* //g' |
    # And add them all back
    sed 's/^/* /g'
}

get_latest_release() {
  curl --silent "https://api.github.com/repos/$1/releases/latest" | # Get latest release from GitHub api
    grep '"tag_name":' |                                            # Get tag line
    sed -E 's/.*"([^"]+)".*/\1/'                                    # Pluck JSON value
}
LATEST_TAG_NAME=$(get_latest_release ComposableFi/composable)

# Returns the last published release on github
# Note: we can't just use /latest because that ignores prereleases
# repo: 'organization/repo'
# Usage: last_github_release "$repo"
last_github_release() {
  i=0
  # Iterate over releases until we find the last release that's not just a draft
  while [ $i -lt 29 ]; do
    out=$(curl -H "Authorization: token $GITHUB_RELEASE_TOKEN" -s "$api_base/$1/releases" | jq ".[$i]")
    echo "$out"
    # Ugh when echoing to jq, we need to translate newlines into spaces :/
    if [ "$(echo "$out" | tr '\r\n' ' ' | jq '.draft')" = "false" ]; then
      echo "$out" | tr '\r\n' ' ' | jq '.tag_name'
      return
    else
      i=$((i + 1))
    fi
  done
}


# Checks whether a tag on github has been verified
# repo: 'organization/repo'
# tagver: 'v1.2.3'
# Usage: check_tag $repo $tagver
check_tag() {
  repo=$1
  tagver=$2
  tag_out=$(curl -H "Authorization: token $GITHUB_RELEASE_TOKEN" -s "$api_base/$repo/git/refs/tags/$tagver")
  tag_sha=$(echo "$tag_out" | jq -r .object.sha)
  object_url=$(echo "$tag_out" | jq -r .object.url)
  if [ "$tag_sha" = "null" ]; then
    return 2
  fi
  verified_str=$(curl -H "Authorization: token $GITHUB_RELEASE_TOKEN" -s "$object_url" | jq -r .verification.verified)
  if [ "$verified_str" = "true" ]; then
    # Verified, everything is good
    return 0
  else
    # Not verified. Bad juju.
    return 1
  fi
}


# Checks whether a given PR has a given label.
# repo: 'organization/repo'
# pr_id: 12345
# label: B1-silent
# Usage: has_label $repo $pr_id $label
has_label() {
  repo="$1"
  pr_id="$2"
  label="$3"

  # These will exist if the function is called in Gitlab.
  # If the function's called in Github, we should have GITHUB_ACCESS_TOKEN set
  # already.
  if [ -n "$GITHUB_RELEASE_TOKEN" ]; then
    GITHUB_TOKEN="$GITHUB_RELEASE_TOKEN"
  elif [ -n "$GITHUB_PR_TOKEN" ]; then
    GITHUB_TOKEN="$GITHUB_PR_TOKEN"
  fi

  out=$(curl -H "Authorization: token $GITHUB_TOKEN" -s "$api_base/$repo/pulls/$pr_id")
  [ -n "$(echo "$out" | tr -d '\r\n' | jq ".labels | .[] | select(.name==\"$label\")")" ]
}

# Formats a message into a JSON string for posting to Matrix
# message: 'any plaintext message'
# formatted_message: '<strong>optional message formatted in <em>html</em></strong>'
# Usage: structure_message $content $formatted_content (optional)
structure_message() {
  if [ -z "$2" ]; then
    body=$(jq -Rs --arg body "$1" '{"msgtype": "m.text", $body}' </dev/null)
  else
    body=$(jq -Rs --arg body "$1" --arg formatted_body "$2" '{"msgtype": "m.text", $body, "format": "org.matrix.custom.html", $formatted_body}' </dev/null)
  fi
  echo "$body"
}

# Post a message to a matrix room
# body: '{body: "JSON string produced by structure_message"}'
# room_id: !fsfSRjgjBWEWffws:matrix.parity.io
# access_token: see https://matrix.org/docs/guides/client-server-api/
# Usage: send_message $body (json formatted) $room_id $access_token
send_message() {
  curl -XPOST -d "$1" "https://matrix.parity.io/_matrix/client/r0/rooms/$2/send/m.room.message?access_token=$3"
}

# Check for runtime changes between two commits. This is defined as any changes
# to runtime/, frame/
has_runtime_changes() {
  from=$1
  to=$2
  if git diff --name-only "${from}...${to}" |
    grep -q -e '^frame/' -e "^runtime/$3/"; then
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

boldprint() { printf "|\n| \033[1m%s\033[0m\n|\n" "${@}"; }
boldcat() {
  printf "|\n"
  while read -r l; do printf "| \033[1m%s\033[0m\n" "${l}"; done
  printf "|\n"
}

# checks if the spec/impl version has increased
check_runtime() {
  VERSIONS_FILE="$1"
  add_spec_version="$(git diff "${LATEST_TAG_NAME}" "${GITHUB_REF_NAME}" -- "${VERSIONS_FILE}" |
    sed -n -r "s/^\+[[:space:]]+spec_version: +([0-9]+),$/\1/p")"
  sub_spec_version="$(git diff "${LATEST_TAG_NAME}" "${GITHUB_REF_NAME}" -- "${VERSIONS_FILE}" |
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

    add_impl_version="$(git diff "${LATEST_TAG_NAME}" "${GITHUB_REF_NAME}" -- "${VERSIONS_FILE}" |
      sed -n -r 's/^\+[[:space:]]+impl_version: +([0-9]+),$/\1/p')"
    sub_impl_version="$(git diff "${LATEST_TAG_NAME}" "${GITHUB_REF_NAME}" -- "${VERSIONS_FILE}" |
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
	- frame/*
	- runtime/$2/*

	versions file: ${VERSIONS_FILE}

	EOT
    return 1
  fi
}
