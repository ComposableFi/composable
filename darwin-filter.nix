{ lib }:
let
  packages = [
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
    "docs-server"
    "docs-static"
    "fmt"
    "frontend-static"
    "frontend-static-persistent"
    "frontend-static-picasso-persistent"
    "frontend-static-firebase"
    "picasso-bench-runtime"
    "picasso-runtime"
    "polkadot-launch"
    "polkadot-node"
    "price-feed"
    "rust-nightly"
    "rust-stable"
    "spell-check"
    "hadolint-check"
    "gex"
    "wasm-optimizer"
  ];

  devShells = [ "minimal" ];
  apps = [ "docs-dev" ];

  # Filter implementation
  darwinSystems = [ "x86_64-darwin" "aarch64-darwin"];
  filterByAllowList = list: lib.filterAttrs (pn: pv: lib.elem pn list);  
  applyAllowList = list: lib.updateManyAttrsByPath (builtins.map (system: { path = [ system ] ; update = filterByAllowList list; }) darwinSystems);

in lib.updateManyAttrsByPath [
  {
    path = [ "packages" ];
    update = applyAllowList packages;
  }
  {
    path = [ "apps" ];
    update = applyAllowList apps;
  }
  {
    path = [ "devShells" ];
    update = applyAllowList devShells;
  }
]
