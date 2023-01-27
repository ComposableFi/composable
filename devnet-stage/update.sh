#!/usr/bin/env nix-shell
#! nix-shell -i bash -p nix-prefetch-scripts jq
REVISION=$1
echo "Revision: ${REVISION}"
URL="https://storage.googleapis.com/composable-binaries/community-releases/picasso/composable-picasso-${REVISION}.tar.gz"
echo "Url: ${URL}"
HASH=$(nix-prefetch-url ${URL})
echo "Hash: ${HASH}"
REVISION_HASH=$(nix-prefetch-git --url https://github.com/ComposableFi/composable --rev ${REVISION} --quiet | jq -r .sha256)
echo "Revision Hash: ${REVISION_HASH}"

echo $(cat devnet.json | jq --arg version ${REVISION} '.composable.version = $version' | jq --arg hash ${HASH} '.composable.hash = $hash'| jq --arg revhash ${REVISION_HASH} '.composable.revhash = $revhash') > devnet.json
