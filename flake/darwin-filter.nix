{ lib }:
let
  nonArmPackages = [ ];

  packages = [
    "acala-node"
    "common-deps"
    "common-deps-nightly"
    "composable-bench-runtime"
    "composable-node"
    "composable-runtime"
    "dali-bench-runtime"
    "dali-runtime"
    "deadnix-check"
    "default"
    "docker-wipe-system"
    "devnet-dali"
    "devnet-dali-centauri-a"
    "devnet-dali-centauri-b"
    "zombienet-rococo-local-dali-dev"
    "docs-server"
    "docs-static"
    "fmt"
    "frontend-static"
    "frontend-static-persistent"
    "frontend-static-picasso-persistent"
    "frontend-static-firebase"
    "picasso-bench-runtime"
    "picasso-runtime"
    "polkadot-node"
    "price-feed"
    "rust-nightly"
    "rust-stable"
    "spell-check"
    "subwasm"
    "hadolint-check"
    "gex"
    "cargo-deny-check"
    "cargo-clippy-check"
    "cargo-fmt-check"
    "cargo-llvm-cov"
    "cargo-udeps-check"
    "devnet-centauri"
  ];

  devShells = [ "minimal" "default" ];
  apps = [ "docs-dev" ];
  nonArmApps = [ ];
  nonArmDevShells = [ ];

  # Filter implementation
  darwinSystems = [ "x86_64-darwin" "aarch64-darwin" ];
  filterByAllowList = list: lib.filterAttrs (pn: pv: lib.elem pn list);
  applyAllowList = list:
    lib.updateManyAttrsByPath (builtins.map (system: {
      path = [ system ];
      update = filterByAllowList list;
    }) darwinSystems);

  # Filter implementation
  # TODO: refactor this nicely
  armSystems = [ "aarch64-linux" ];
  filterByBlockList = list: lib.filterAttrs (pn: pv: !(lib.elem pn list));
  applyBlockList = list:
    lib.updateManyAttrsByPath (builtins.map (system: {
      path = [ system ];
      update = filterByBlockList list;
    }) armSystems);

in lib.updateManyAttrsByPath [
  {
    path = [ "packages" ];
    update = x: applyBlockList nonArmPackages (applyAllowList packages x);
  }
  {
    path = [ "apps" ];
    update = x: applyBlockList nonArmApps (applyAllowList apps x);
  }
  {
    path = [ "devShells" ];
    update = x: applyBlockList nonArmDevShells (applyAllowList devShells x);
  }
]
