# validates root nix
nix flake check --no-update-lock-file --keep-going
nix flake show --allow-import-from-derivation --show-trace
