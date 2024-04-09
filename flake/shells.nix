{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, systemCommonRust, centauri
    , bashTools, osmosis, ... }:
    let
      networks = pkgs.networksLib;
      env = {
        LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath
          (with pkgs; [ stdenv.cc.cc.lib llvmPackages.libclang.lib pkgs.zlib ]);
        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
        PROTOC = "${pkgs.protobuf}/bin/protoc";
        ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
        NIX_PATH = "nixpkgs=${pkgs.path}";
      };
      tools = with pkgs;
        with self'.packages; [
          bacon
          binaryen
          bun
          centaurid
          clang
          dasel
          forge
          gex
          git
          git-lfs
          grpcurl
          jq
          lldb
          llvmPackages_latest.bintools
          llvmPackages_latest.lld
          llvmPackages_latest.llvm
          neutrond
          nix-tree
          nixfmt
          nodejs
          nodePackages.npm
          nodePackages.typescript
          nodePackages.typescript-language-server
          openssl
          openssl.dev
          pkg-config
          process-compose
          protobuf
          python3
          qemu
          rnix-lsp
          rust-nightly
          sad
          self.inputs.devenv.packages.${system}.devenv
          subwasm
          taplo
          typescript
          websocat
          xorriso
          yarn
          zlib.out
          zombienet
        ];
      all-deps-attrs = {
        inherit pkgs;
        inputs = self.inputs;
        modules = [{
          packages = tools;
          devcontainer.enable = false;
          inherit env;
          enterShell = ''
          '';

        }];
      };
    in {
      packages = {
        all-deps-shell = pkgs.linkFarmFromDrvs "all-deps-shell" tools;
      };
      devShells = {
        default = self.inputs.devenv.lib.mkShell all-deps-attrs;
      };
    };
}
