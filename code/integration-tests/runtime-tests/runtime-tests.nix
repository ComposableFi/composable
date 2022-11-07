{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    packages = rec {
      runtime-tests = pkgs.stdenv.mkDerivation {
        name = "runtime-tests";
        src =
          builtins.filterSource (path: _type: baseNameOf path != "node_modules")
          ./.;
        dontUnpack = true;
        installPhase = ''
          mkdir $out/
          cp -r $src/. $out/
        '';
      };

      prettier-check = pkgs.stdenv.mkDerivation {
        name = "prettier-check";
        dontUnpack = true;
        buildInputs = [ pkgs.nodePackages.prettier runtime-tests ];
        installPhase = ''
          mkdir $out
          prettier \
          --config="${runtime-tests}/.prettierrc" \
          --ignore-path="${runtime-tests}/.prettierignore" \
          --check \
          --loglevel=debug \
          ${runtime-tests}
        '';
      };
    };
  };
}
