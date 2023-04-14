{ self, withSystem, inputs, lib, options, flake-parts-lib, specialArgs, config
}: {
  flake = {
    composable = {
      shell = ''
        export PROTOC="${inputs.nixpkgs.legacyPackages.x86_64-linux.protobuf}/bin/protoc";
        export ROCKSDB_LIB_DIR="${inputs.nixpkgs.legacyPackages.x86_64-linux.rocksdb}/lib";
        export LIBCLANG_PATH="${inputs.nixpkgs.legacyPackages.x86_64-linux.llvmPackages.libclang.lib}/lib";
      '';
    };
    homeConfigurations = let user = "vscode";
    in (withSystem "x86_64-linux"
      ({ config, self', inputs', pkgs, devnetTools, this, subnix, ... }: {
        vscode = let codespace = with pkgs; [ cachix acl direnv ];
        in self.inputs.home-manager.lib.homeManagerConfiguration {
          inherit pkgs;
          modules = [{
            home = {
              username = user;
              sessionVariables = subnix.subenv;
              homeDirectory = "/home/${user}";
              stateVersion = "22.11";
              packages = codespace;
              # packages = with pkgs;
              #   with self'.packages;
              #   [ clang nodejs python3 yarn sad git git-lfs subwasm zombienet ]
              #   ++ (with self'.packages; [ rust-nightly ]) ++ codespace;
            };
            programs = {
              home-manager.enable = true;
              # direnv = {
              #   enable = true;
              #   nix-direnv = { enable = true; };
              # };
            };
          }];
        };
      })) // { };
  };
}
