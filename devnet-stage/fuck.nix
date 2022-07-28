with (import <nixpkgs>{});
stdenv.mkDerivation {
  name = "fuck";
  buildInputs = [ python3];
  installPhase = ''
  mkdir -p $out/bin;
  echo "42" > $out/bin/fuck.sh
  '';
}

#  nix-env -iA nixos.uutils-coreutils