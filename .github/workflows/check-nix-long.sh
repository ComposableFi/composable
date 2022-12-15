NIX_DEBUG_COMMAND="" && [ $ACTIONS_RUNNER_DEBUG -eq "true" ] && NIX_DEBUG_COMMAND='--print-build-logs --debug --show-trace --verbose'
set -o pipefail
NIXPKGS_ALLOW_BROKEN=1 nix flake check --keep-going --no-build --allow-import-from-derivation  --no-update-lock-file --fallback -L ${NIX_DEBUG_COMMAND} --impure --option sandbox relaxed --impure 2>&1 || true | tee "nix.check.log"  || true
set +o pipefail
# https://github.com/NixOS/nix/issues/7464
EXIT=$(cat "nix.check.log" | grep --invert-match '^error (ignored): error:' | grep --invert-match '^error: some errors were encountered during the evaluation' | grep -c 'error:')
echo $EXIT