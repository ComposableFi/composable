# TODO: move to `parachains` folder
{ pkgs, rust-overlay }:
with pkgs.stdenv;
with pkgs;

mkDerivation {
  name = "acala";
  src = pkgs.fetchgit {
    url = "https://github.com/AcalaNetwork/Acala.git";
    rev = "e9d2b3caa0663c1d3e7d4d6e7d3faef4a569099c";
    sha256 = "sha256-Cw/92L51P1LmQ34He/7+76pffUz3uU4Tlrt3kd5hNQk=";
    fetchSubmodules = true;
    deepClone = true;
  };

  configurePhase = "git submodule update --init --recursive";
  installPhase = ''
    mkdir --parents $out/bin && mv ./target/production/acala $out/bin
  '';

  # substrate-attrs-node-with-attrs
  __noChroot = true;
  doCheck = false;
  buildInputs = [ openssl ];
  nativeBuildInputs = [ clang git rust-overlay ];
  buildPhase = ''
    		mkdir home
        export HOME=$PWD/home	
    	  make build-release 
    	'';
  # TODO: moved these to some `cumulus based derivation`
  LD_LIBRARY_PATH =
    lib.strings.makeLibraryPath [ stdenv.cc.cc.lib llvmPackages.libclang.lib ];
  LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
}
