{ self, ... }: {
  perSystem = { config, self', inputs', system, pkgs, ... }: {
    packages = {
      up = pkgs.writeShellApplication {
        name = "up";
        text = ''
          nix flake lock --update-input networks
          nix flake lock --update-input cosmos
          nix flake lock --update-input cvm
        '';
      };
    };
    _module.args.pkgs = import self.inputs.nixpkgs {
      inherit system;
      overlays = with self.inputs; [
        npm-buildpackage.overlays.default
        polkadot.overlays.default
        rust-overlay.overlays.default
        zombienet.overlays.default
        process-compose.overlays.default
        networks.overlays.default
        sbt-derivation.overlays.default
      ];
    };
  };
}
