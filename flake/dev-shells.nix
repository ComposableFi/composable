{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, systemCommonRust, ... }:
    let
      env = {
        LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath
          (with pkgs; [ stdenv.cc.cc.lib llvmPackages.libclang.lib pkgs.zlib ]);
        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
        PROTOC = "${pkgs.protobuf}/bin/protoc";
        ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
        NIX_PATH = "nixpkgs=${pkgs.path}";
      };
      tools = with pkgs;
        with self'.packages;
        [
          binaryen
          clang
          dasel
          git
          git-lfs
          grpcurl
          jq
          nix-tree
          nixfmt
          nodejs
          openssl
          process-compose
          python3
          rnix-lsp
          sad
          self.inputs.cosmos.packages.${system}.cosmwasm-check
          self.inputs.cosmos.packages.${system}.gex
          subwasm
          terraform
          terraform-ls
          websocat
          yarn
          zombienet
        ] ++ (with self'.packages; [ rust-nightly ]);
      defaultattrs = {
        inherit pkgs;
        inputs = self.inputs;
        modules = [{
          packages = tools;
          devcontainer.enable = true;
          inherit env;
        }];
      };
      allattrs = defaultattrs // {
        modules = [{
          packages = tools ++ (with pkgs;
            with self'.packages; [
              bacon
              devenv
              google-cloud-sdk
              lldb
              llvmPackages_latest.bintools
              llvmPackages_latest.lld
              llvmPackages_latest.llvm
              nodePackages.typescript
              nodePackages.typescript-language-server
              openssl
              openssl.dev
              pkg-config
              qemu
              taplo
              xorriso
              zlib.out
            ]);
          devcontainer.enable = true;
          inherit env;
        }];
      };
    in {
      packages = {
        devenv = self.inputs.devenv.packages.${system}.devenv;
        devprofile = pkgs.linkFarmFromDrvs "devprofile" tools;
      };
      devShells = {
        default = self.inputs.devenv.lib.mkShell defaultattrs;
        all = self.inputs.devenv.lib.mkShell allattrs;
        xc = pkgs.mkShell {
          buildInputs = tools ++ (with self'.packages; [ centaurid ]);
        };
      };
    };
}
