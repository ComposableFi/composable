# Overview

Configurations external to this repository checkout, either 3rd party or not in this repo composable or not this revision of repo.

3rd parties either do not have nix configurations or we need heavily patch them.

Unit test can be run with flake here. Unit tests run fast and do not depend on remote.


## Guidelines

When `import` or `callPackage` do not suffix it with `.nix` as it allows to expand any file to folder with `default.nix`.

Variables which are input from external non `nix` files (examples, json/yaml/toml) to be suffixed with `-input`. Inputs prevent early validation of packages without instationation.  
