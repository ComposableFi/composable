{ self, ... }: {
  flake = {
    # NOTE: These will bue put in a part soon.
    overlays = {
      default = let
        mkDevnetProgram = { pkgs }:
          name: spec:
          pkgs.writeShellApplication {
            inherit name;
            runtimeInputs = [ pkgs.arion pkgs.docker pkgs.coreutils pkgs.bash ];
            text = ''
              arion --prebuilt-file ${
                pkgs.arion.build spec
              } up --build --force-recreate -V --always-recreate-deps --remove-orphans
            '';
          };
      in self.inputs.nixpkgs.lib.composeManyExtensions [
        self.inputs.arion-src.overlays.default
        (final: _prev: {
          composable = {
            mkDevnetProgram = final.callPackage mkDevnetProgram { };
          };
        })
      ];
    };
  };
}
