{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, systemCommonRust, centauri
    , osmosis, ... }:
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
          protobuf
          python3
          rnix-lsp
          sad
          gex
          bech32cli
          subwasm
          terraform
          terraform-ls
          websocat
          yarn
          zombienet
          self'.packages.bech32cli
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
        centauri-testnet = self.inputs.devenv.lib.mkShell {
          inherit pkgs;
          inputs = self.inputs;
          modules = [{
            packages = [ self'.packages.centaurid ];
            env = {
              FEE = "ppica";
              NETWORK_ID = 2;
              CHAIN_ID = "banksy-testnet-3";
              DIR = ".centaurid";
              BINARY = "centaurid";
              NODE = "https://rpc-t.composable.nodestake.top:443";
              INTERPRETER_WASM_FILE =
                "${self'.packages.xc-cw-contracts}/lib/cw_xc_interpreter.wasm";
              GATEWAY_WASM_FILE =
                "${self'.packages.xc-cw-contracts}/lib/cw_xc_gateway.wasm";
            };
          }];
        };

        centauri-mainnet = self.inputs.devenv.lib.mkShell {
          inherit pkgs;
          inputs = self.inputs;
          modules = [{
            packages = [ self'.packages.centaurid ];
            env = centauri.env.mainnet // {
              INTERPRETER_WASM_FILE =
                "${self'.packages.xc-cw-contracts}/lib/cw_xc_interpreter.wasm";
              GATEWAY_WASM_FILE =
                "${self'.packages.xc-cw-contracts}/lib/cw_xc_gateway.wasm";
            };
          }];
        };

        osmosis-testnet = self.inputs.devenv.lib.mkShell {
          inherit pkgs;
          inputs = self.inputs;
          modules = [{
            packages = [ self'.packages.osmosisd ];
            env = {
              FEE = "uosmo";
              NETWORK_ID = 3;
              CHAIN_ID = "osmo-test-5";
              DIR = ".osmosisd";
              BINARY = "osmosisd";
              NODE = "https://rpc.testnet.osmosis.zone:443";
              INTERPRETER_WASM_FILE =
                "${self'.packages.xc-cw-contracts}/lib/cw_xc_interpreter.wasm";
              GATEWAY_WASM_FILE =
                "${self'.packages.xc-cw-contracts}/lib/cw_xc_gateway.wasm";
            };
          }];
        };
      };
    };
}
