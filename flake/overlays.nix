{ self, ... }: {
  perSystem = { config, self', inputs', system, ... }: {
    # this is very bad hack, it kills other usage of overlays in dependant flakes
    # so need manuall add their overlays
    # not yet checked how to fix
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
