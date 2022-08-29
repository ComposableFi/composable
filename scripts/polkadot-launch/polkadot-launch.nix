{ pkgs }:
let
  src = pkgs.fetchFromGitHub {
    owner = "paritytech";
    repo = "polkadot-launch";
    rev = "951af7055e2c9abfa7a03ee7848548c1a3efdc16";
    hash = "sha256-ZaCHgkr5lVsGFg/Yvx6QY/zSiIafwSec+oiioOWTZMg=";
  };
in pkgs.mkYarnPackage {
  name = "polkadot-launch";
  inherit src;
  packageJSON = "${src}/package.json";
  yarnLock = "${src}/yarn.lock";
  buildPhase = ''
    yarn build
  '';
  distPhase = "true";
  postInstall = ''
    chmod +x $out/bin/polkadot-launch
  '';
}
