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
        [ clang nodejs python3 yarn sad git git-lfs subwasm ]
        ++ (with self'.packages; [ rust-nightly ]);
      defaultattrs = {
        inherit pkgs;
        inputs = self.inputs;
        modules = [{
          packages = tools;
          devcontainer.enable = true;
          inherit env;
        }];
      };
      cosmosattrs = defaultattrs // {
        modules = [{
          packages = tools ++ (with self'.packages; [ junod gex ]);
          devcontainer.enable = true;
          inherit env;
          enterShell = ''
            echo "junod alice key:"
            echo "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose" | junod keys add alice --recover --keyring-backend test || true
          '';
        }];
      };
      allattrs = defaultattrs // {
        modules = [{
          packages = tools ++ (with pkgs;
            with self'.packages; [
              bacon
              google-cloud-sdk
              jq
              lldb
              llvmPackages_latest.bintools
              llvmPackages_latest.lld
              llvmPackages_latest.llvm
              nix-tree
              nixfmt
              openssl
              openssl.dev
              pkg-config
              qemu
              rnix-lsp
              taplo
              xorriso
              zlib.out
              nix-tree
              nixfmt
              rnix-lsp
              nodePackages.typescript
              nodePackages.typescript-language-server
              binaryen
              gex
            ]);
          devcontainer.enable = true;
          inherit env;
        }];
      };
    in {
      devShells = {
        default = self.inputs.devenv.lib.mkShell defaultattrs;
        cosmos = self.inputs.devenv.lib.mkShell cosmosattrs;
        all = self.inputs.devenv.lib.mkShell allattrs;
      };
    };
}
