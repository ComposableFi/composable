# TODO: move to `parachains` folder
{ pkgs, rust-overlay }:
with pkgs.stdenv;
with pkgs;

mkDerivation {
  name = "acala";
  src = pkgs.fetchgit {
    url = "https://github.com/AcalaNetwork/Acala.git";
    rev = "e9d2b3caa0663c1d3e7d4d6e7d3faef4a569099c";
    sha256 = "sha256-buRxUVdyMIAg/FFi/McTbYvGSk8LM7v+HQ09YGSo2dk=";
    fetchSubmodules = true;
    deepClone = true;
  };
  # TODO: unify with other networks build
  __noChroot = true;
  doCheck = false;
  configurePhase = "git submodule update --init --recursive";
  installPhase =
    "	mkdir --parents $out/bin && mv ./target/production/acala $out/bin\n";
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
