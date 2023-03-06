{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    packages = 
      let pkgs' = import self.inputs.nixpkgs {
        inherit system;
        overlays = [
          self.inputs.npm-buildpackage.overlays.default
        ];
      };
      in {
        remark = pkgs'.buildYarnPackage {
          src =
            builtins.filterSource (path: _type: baseNameOf path != "node_modules")
            ./.;
          dontNpmPrune = true;
        };
      };
  };
}
