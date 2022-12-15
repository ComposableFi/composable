NIX_DEBUG_COMMAND="" && [ $ACTIONS_RUNNER_DEBUG == "true" ] && NIX_DEBUG_COMMAND='--print-build-log --debug --show-trace'
NIXPKGS_ALLOW_BROKEN=1 nix flake check --keep-going --no-build --allow-import-from-derivation  --no-update-lock-file --fallback -L ${NIX_DEBUG_COMMAND} --impure --option sandbox relaxed
