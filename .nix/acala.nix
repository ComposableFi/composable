{pkgs, }:
with pkgs.stdenv;
with pkgs;

mkDerivation {
	name = "acala";
  src = pkgs.fetchFromGitHub {
                owner = "AcalaNetwork";
                repo = "Acala";
                rev = "e9d2b3caa0663c1d3e7d4d6e7d3faef4a569099c";
                hash = "sha256-7oQxYDX+hMSrXZdwWbF1tkahSr1olze1HaCOjCvfWsc=";
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
  