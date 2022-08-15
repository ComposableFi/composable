{
  description = "Our overrides for ci on linux";
  outputs = { self }: {
    lib = {
      eachDefaultSystem = f:
        let
          system = "x86_64-linux";
          outputs = f system;
          appendSystem = attrs: key:
            # https://github.com/numtide/flake-utils/issues/77
            if key == "nixopsConfigurations" then
              { ${key} = outputs.${key}; } // attrs
            else
              { ${key} = { ${system} = outputs.${key}; }; } // attrs;
          # maps `packages.foobar`
          # into `packages.${system}.foobar`
        in builtins.foldl' appendSystem { } (builtins.attrNames outputs);
      # Returns the structure used by `nix app`
      mkApp = { drv, name ? drv.pname or drv.name
        , exePath ? drv.passthru.exePath or "/bin/${name}" }: {
          type = "app";
          program = "${drv}${exePath}";
        };
    };
  };
}
