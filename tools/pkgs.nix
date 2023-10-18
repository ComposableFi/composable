{ self, ... }: {
  perSystem = { config, self', inputs', system, ... }: {
    _module.args.pkgs = import self.inputs.nixpkgs {
      inherit system;
      overlays = with self.inputs; [
        npm-buildpackage.overlays.default
        polkadot.overlays.default
        rust-overlay.overlays.default
        zombienet.overlays.default
      ];
    };
  };
}
