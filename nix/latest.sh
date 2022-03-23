#!/usr/bin/env nix-shell
#! nix-shell -i bash -p nix-prefetch-scripts jq
REVISION=$(git rev-parse origin/main)
echo "Revision: ${REVISION}"
URL="https://storage.googleapis.com/composable-binaries/community-releases/picasso/composable-picasso-${REVISION}.tar.gz"
echo "Url: ${URL}"
HASH=$(nix-prefetch-url ${URL})
echo "Hash: ${HASH}"
echo $(cat devnet.json | jq --arg version ${REVISION} '.composable.version = $version' | jq --arg hash ${HASH} '.composable.hash = $hash') > devnet.json
nix develop --command run-$1
