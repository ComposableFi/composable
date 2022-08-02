# https://github.com/NixOS/nix/issues/5560
nix flake show --allow-import-from-derivation --show-trace --override-input flake-utils ./.nix/override-input/flake-utils/   --show-trace
# with default systems errors with
# auto-patchelf-hook, spidermonkey-78.15.0  is not supported on ‘aarch64-darwin’
# asks to set export NIXPKGS_ALLOW_UNSUPPORTED_SYSTEM=1
# and openjdk-headless-16+36 is too old 
# asks to set export NIXPKGS_ALLOW_INSECURE=1
# not clear why there are such weird deps nor how to override configs in nix, not via export env
nix flake check --override-input flake-utils ./.nix/override-input/flake-utils/ --no-update-lock-file --keep-going
# TODO: on it too for fail fast before build
# nixops deploy --dry-run