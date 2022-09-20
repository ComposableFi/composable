with (import <nixpkgs> {});
rec {
  remark = mkYarnPackage {
    name = "remark";
    src = ./.;
    packageJSON = ./package.json;
    yarnLock = ./yarn.lock;
    yarnNix = ./yarn.nix;
  };
}
