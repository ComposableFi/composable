{ self, ... }: {
  perSystem = { config, self', inputs', system, ... }: {
    _module.args.pkgs = import self.inputs.nixpkgs {
      inherit system;
      overlays = with self.inputs; [
        self.overlays.default
        npm-buildpackage.overlays.default
        rust-overlay.overlays.default
        zombienet.overlays.default
      ];
    };
    # remove me when the `nixops_unstable` works again on the latest unstable
    _module.args.pkgs-working-nixops =
      import self.inputs.nixpkgs-working-nixops { inherit system; };
  };
}
