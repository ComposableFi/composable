{pkgs, }:
with pkgs.stdenv;
mkDerivation {
  src = pkgs.fetchFromGitHub {
                owner = "AcalaNetwork";
                repo = "Acala";
                rev = "e9d2b3caa0663c1d3e7d4d6e7d3faef4a569099c";
                hash = "sha256-ggcyOsA4cyo5l87cZmOMI0w1gCzmWy9NRJiWxjBdB1E=";
              };     
}
	# (
	# 	cd ../../..
	# 	git clone https://github.com//.git
	# 	cd Acala
	# 	make init
	# 	make build-release
	# 	./target/production/acala --version
	# )

  