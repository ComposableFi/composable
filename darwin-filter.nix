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
    "default"
    "devnet-dali"
    "docs-server"
    "docs-static"
    "fmt"
    "picasso-bench-runtime"
    "picasso-runtime"
    "polkadot-launch"
    "polkadot-node"
    "rust-nightly"
    "rust-stable"
    "spell-check"
    "wasm-optimizer"
  ];

  devShells = [ "minimal" ];

  apps = [ "docs-dev" ];


  # Filter implementation
  darwinSystems = [ "x86_64-darwin" "aarch64-darwin"];
  filterByAllowlist = list: lib.filterAttrs (pn: pv: lib.elem pn list);  
  applyAllowList = list: lib.updateManyAttrsByPath (builtins.map (system: { path = [ system ] ; update = filterByAllowList list }) darwinSystems);

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
