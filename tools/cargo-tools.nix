{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    _module.args.cargoTools = rec {
      rustSrc = pkgs.lib.cleanSourceWith {
        filter = pkgs.lib.cleanSourceFilter;
        src = pkgs.lib.cleanSourceWith {
          filter = let
            hasSuffix = pkgs.lib.strings.hasSuffix;
            isProto = name: type: type == "regular" && hasSuffix ".proto" name;
            isJSON = name: type: type == "regular" && hasSuffix ".json" name;
            isREADME = name: type:
              type == "regular" && hasSuffix "README.md" name;
            isDir = name: type: type == "directory";
            isCargo = name: type:
              type == "regular"
              && (hasSuffix ".toml" name || hasSuffix ".lock" name);
            isRust = name: type: type == "regular" && hasSuffix ".rs" name;
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
          [ ./../.gitignore ] ./../code/.;
          src = ./../code/.;
        };
      };
    };
  };
}
