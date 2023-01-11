{ self, ... }: {
  perSystem = { config, self', inputs', system, ... }: {
    _module.args.pkgs = import self.inputs.nixpkgs {
      inherit system;
      overlays = with self.inputs; [
        self.overlays.default
        npm-buildpackage.overlays.default
        rust-overlay.overlays.default
      ];
    };
  };
}
