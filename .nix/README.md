# Overview

Configurations external to this repository checkout, either 3rd party or not in this repo composable or not this revision of repo.

3rd parties either do not have nix configurations or we need heavily patch them.

Unit test can be run with flake here. Unit tests run fast and do not depend on remote.


## Guidelines

Variables which are input from external non `nix` files (examples, json/yaml/toml) to be suffixed with `-input`. Inputs prevent early validation of packages without instantiation.  

Scrips are located in folders on which they act upon, until scope is multifolder or not scoped.
## Source or binary for Rust deps?

Binary may not output all targets, not all commits, not forks and hard to diff changes from release to release and audit security.

Rust can do `cargo install` which is the same as `crane` does - builds from sources.

So for rust dependencies, we use source by default.

Because we use cachix, build as fast as binary.

Also Rust(and Go) are projects with tendency toward deterministic builds.