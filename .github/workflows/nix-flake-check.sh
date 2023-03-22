NIX_DEBUG_COMMAND="" && [[ $ACTIONS_RUNNER_DEBUG = "true" ]] && NIX_DEBUG_COMMAND='--print-build-logs --debug --show-trace --verbose'
set -o pipefail -o errexit
NIXPKGS_ALLOW_BROKEN=1 nix flake check --keep-going --no-build --allow-import-from-derivation  --no-update-lock-file --accept-flake-config --fallback -L ${NIX_DEBUG_COMMAND} --impure --option sandbox relaxed --impure 2>&1 | tee "nix.check.log"  || true
set +o pipefail +o errexit
echo "exited with(https://github.com/NixOS/nix/issues/7464) ${$?}" 
cat "nix.check.log" | grep --invert-match  "error: path [']/nix/store/[a-zA-Z0-9]\+-[a-zA-Z0-9\.-]\+['] is not valid" \
| grep --invert-match  "error: cannot substitute path [']/nix/store/[a-zA-Z0-9]\+-[a-zA-Z0-9\.-]\+['] \- no write access to the Nix store" \
| grep --invert-match '^error: some errors were encountered during the evaluation' > "filtered.nix.check.log"
RESULT=$(cat "filtered.nix.check.log" | grep -c 'error:')
echo "Got errors $RESULT"
if [[ $RESULT != 0 ]]; then exit $RESULT; fi