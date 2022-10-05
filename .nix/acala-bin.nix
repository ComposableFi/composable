{ pkgs, rust-nightly }:
let substrate-attrs = import ./substrate.nix { inherit rust-nightly pkgs; };
in with pkgs.stdenv;
with pkgs;
mkDerivation (substrate-attrs // {
  name = "acala";
  src = pkgs.fetchgit {
    url = "https://github.com/AcalaNetwork/Acala.git";
    rev = "368a6bc089534031ef2671a3071ba89720f40be7";
    sha256 = "sha256-IERbXtk0zfXOSfJPrZsmmqhcwzLbaknwH4ULaaXImJM=";
    fetchSubmodules = true;
    deepClone = true;
  };

  configurePhase = "git submodule update --init --recursive";
  installPhase = ''
    mkdir --parents $out/bin && mv ./target/production/acala $out/bin
  '';
  buildPhase = ''
    		mkdir home
        export HOME=$PWD/home	
    	  make build-release 
    	'';
})
