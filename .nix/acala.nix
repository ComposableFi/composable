{pkgs, }:
with pkgs.stdenv;
with pkgs;

mkDerivation {
	name = "acala";
	# NOTE:
	# seems make of acala is too active, having all thing to get all does not helps
	# 6] Couldn't resolve host name (Could not resolve host: github.com); class=Net (12
	src = pkgs.fetchgit {
    url = "https://github.com/AcalaNetwork/Acala.git";
		rev = "e9d2b3caa0663c1d3e7d4d6e7d3faef4a569099c";
		fetchSubmodules = true;
		deepClone = true;
  };	

	installPhase = ''
	  ls
		mkdir --parents $out/bin && make init
	'';
	buildInputs = [ openssl ];
	nativeBuildInputs = [ clang cargo ];
	buildPhase = ''
	  ls
		make build-release && mv ./target/production/acala $out/bin
	'';
	# TODO: moved these to some `cumulus based derivation`
	LD_LIBRARY_PATH = lib.strings.makeLibraryPath [
		stdenv.cc.cc.lib
		llvmPackages.libclang.lib
	];
	LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";	
}
  