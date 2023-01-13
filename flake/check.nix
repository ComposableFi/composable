{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    packages = {
      check = let
        checks = [
          "spell-check"
          "nixfmt-check"
          "hadolint-check"
          "deadnix-check"
          "docs-static"
        ];
        toCommand = check: ''
                  echo "Checking ${check}..."
          				nix build .\#${check} --no-warn-dirty
          			'';
        script = pkgs.lib.concatMapStrings toCommand checks;
      in pkgs.writeShellScriptBin "check" script;
    };
  };
}
