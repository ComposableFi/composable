{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    _module.args.cargoTools = rec {
      mkRustSrc = root:
        pkgs.lib.cleanSourceWith {
          filter = pkgs.lib.cleanSourceFilter;
          src = pkgs.lib.cleanSourceWith {
            filter = let
              isProto = name: type:
                type == "regular" && pkgs.lib.strings.hasSuffix ".proto" name;
              isJSON = name: type:
                type == "regular" && pkgs.lib.strings.hasSuffix ".json" name;
              isREADME = name: type:
                type == "regular"
                && pkgs.lib.strings.hasSuffix "README.md" name;
              isDir = name: type: type == "directory";
              isCargo = name: type:
                type == "regular" && pkgs.lib.strings.hasSuffix ".toml" name
                || type == "regular" && pkgs.lib.strings.hasSuffix ".lock" name;
              isRust = name: type:
                type == "regular" && pkgs.lib.strings.hasSuffix ".rs" name;
              customFilter = name: type:
                builtins.any (fun: fun name type) [
                  isCargo
                  isRust
                  isDir
                  isREADME
                  isJSON
                  isProto
                ];
            in pkgs.nix-gitignore.gitignoreFilterPure customFilter
            [ ../.gitignore ] root;
            src = root;
          };
        };

      rustSrc = mkRustSrc ./../code/.;
    };
  };
}
