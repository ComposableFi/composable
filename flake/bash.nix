{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    _module.args.bashTools = rec {
      export = env:
        builtins.foldl' (a: b: "${a}${b}") "" (pkgs.lib.mapAttrsToList
          (name: value: "export ${name}=${builtins.toString value};") env);
    };
  };
}
