{ lib }:
let
  packages = [ "docs-static" "docs-server" "spell-check" "devnet-dali" "fmt" ];

  devShells = [ "minimal" ];

  apps = [ "docs-dev" ];

  isDarwin = sys: lib.elem sys [ "x86_64-darwin" "aarch64-darwin" ];

  applyAllowList = allowList:
    lib.mapAttrs (sn: sv:
      if isDarwin sn then
        lib.filterAttrs (pn: pv: lib.elem pn allowList) sv
      else
        sv);

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
