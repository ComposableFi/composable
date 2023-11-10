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
          bech32cli
          binaryen
          bun
          clang
          dasel
          forge
          gex
          git
          git-lfs
          grpcurl
          jq
          nix-tree
          nixfmt
          nixos-rebuild
          nodejs
          nodePackages.npm
          openssl
          process-compose
          protobuf
          python3
          rnix-lsp
          sad
          self'.packages.bech32cli
          subwasm
          opentofu
          terraform-ls
          typescript
          websocat
          yarn
          zombienet
        ] ++ (with self'.packages; [ rust-nightly ]);
      defaultattrs = {
        inherit pkgs;
        inputs = self.inputs;
        modules = [{
          packages = tools;
          devcontainer.enable = false;
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
          devcontainer.enable = false;
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
        centauri-devnet = self.inputs.devenv.lib.mkShell {
          inherit pkgs;
          inputs = self.inputs;
          modules = [rec {
            packages = [ self'.packages.centaurid ];
            env = {
              FEE = "ppica";
              NETWORK_ID = 2;
              CHAIN_ID = "centauri-dev";
              DIR = "devnet/.centaurid";
              BINARY = "centaurid";
              NODE = "tcp://localhost:26657";
              INTERPRETER_WASM_FILE =
                "${self'.packages.xc-cw-contracts}/lib/cw_xc_interpreter.wasm";
              GATEWAY_WASM_FILE =
                "${self'.packages.xc-cw-contracts}/lib/cw_xc_gateway.wasm";
            };
            enterShell = ''
              rm --force --recursive ~/.centauri
              mkdir --parents ~/.centauri/config
              echo 'keyring-backend = "test"' >> ~/.centauri/config/client.toml
              echo 'output = "json"' >> ~/.centauri/config/client.toml
              echo 'node = "${env.NODE}"' >> ~/.centauri/config/client.toml
              echo 'chain-id = "${env.CHAIN_ID}"' >> ~/.centauri/config/client.toml               
              echo "apart ahead month tennis merge canvas possible cannon lady reward traffic city hamster monitor lesson nasty midnight sniff enough spatial rare multiply keep task" | "$BINARY" keys add cvm-admin --recover --keyring-backend test --output json            
            '';
          }];
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
              DIR = "testnet/.centaurid";
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
              EXECUTOR_WASM_FILE =
                "${self'.packages.xc-cw-contracts}/lib/cw_xc_interpreter.wasm";
              GATEWAY_WASM_FILE =
                "${self'.packages.xc-cw-contracts}/lib/cw_xc_gateway.wasm";
              FEE = "ppica";
            };

            enterShell = ''
              rm --force --recursive ~/.centauri
              mkdir --parents ~/.centauri/config
              echo 'keyring-backend = "os"' >> ~/.centauri/config/client.toml
              echo 'output = "json"' >> ~/.centauri/config/client.toml
              echo 'node = "https://rpc-composable-ia.cosmosia.notional.ventures:443"' >> ~/.centauri/config/client.toml
              echo 'chain-id = "centauri-1"' >> ~/.centauri/config/client.toml
            '';
          }];
        };

        osmosis-mainnet = self.inputs.devenv.lib.mkShell {
          inherit pkgs;
          inputs = self.inputs;
          modules = [{
            packages = [ self'.packages.osmosisd ];
            env = osmosis.env.mainnet // {
              INTERPRETER_WASM_FILE =
                "${self'.packages.xc-cw-contracts}/lib/cw_xc_interpreter.wasm";
              GATEWAY_WASM_FILE =
                "${self'.packages.xc-cw-contracts}/lib/cw_xc_gateway.wasm";
            };
            enterShell = ''
              rm ~/.osmosisd/config/client.toml 
              osmosisd set-env mainnet
            '';
          }];
        };

        osmosis-testnet = self.inputs.devenv.lib.mkShell {
          inherit pkgs;
          inputs = self.inputs;
          modules = [{
            packages = [ self'.packages.osmosisd ];
            env = osmosis.env.testnet // {
              INTERPRETER_WASM_FILE =
                "${self'.packages.xc-cw-contracts}/lib/cw_xc_interpreter.wasm";
              GATEWAY_WASM_FILE =
                "${self'.packages.xc-cw-contracts}/lib/cw_xc_gateway.wasm";
              FEE = "uatom";
            };
          }];
        };

        osmosis-local = self.inputs.devenv.lib.mkShell {
          inherit pkgs;
          inputs = self.inputs;
          modules = [rec {
            packages = [ self'.packages.osmosisd ];
            env = osmosis.env.testnet // {
              INTERPRETER_WASM_FILE =
                "${self'.packages.xc-cw-contracts}/lib/cw_xc_interpreter.wasm";
              GATEWAY_WASM_FILE =
                "${self'.packages.xc-cw-contracts}/lib/cw_xc_gateway.wasm";
              NODE = "tcp://localhost:36657";
              FEE = "uatom";
            };
            enterShell = ''
              osmosisd set-env localnet
              echo 'chain-id = "osmosis-dev"' > ~/.osmosisd-local/config/client.toml 
              echo 'keyring-backend = "test"' >> ~/.osmosisd-local/config/client.toml 
              echo 'output = "json"' >> ~/.osmosisd-local/config/client.toml 
              echo 'broadcast-mode = "block"' >> ~/.osmosisd-local/config/client.toml 
              echo 'human-readable-denoms-input = false' >> ~/.osmosisd-local/config/client.toml 
              echo 'human-readable-denoms-output = false' >> ~/.osmosisd-local/config/client.toml 
              echo 'gas = ""' >> ~/.osmosisd-local/config/client.toml 
              echo 'gas-prices = ""' >> ~/.osmosisd-local/config/client.toml 
              echo 'gas-adjustment = ""' >> ~/.osmosisd-local/config/client.toml 
              echo 'fees = ""' >> ~/.osmosisd-local/config/client.toml 
              echo 'node = "${env.NODE}"' >> ~/.osmosisd-local/config/client.toml 
            '';
          }];
        };

        osmosis-devnet = self.inputs.devenv.lib.mkShell {
          inherit pkgs;
          inputs = self.inputs;
          modules = [rec {
            packages = [ self'.packages.osmosisd ];
            env = osmosis.env.remote-devnet // {
              INTERPRETER_WASM_FILE =
                "${self'.packages.xc-cw-contracts}/lib/cw_xc_interpreter.wasm";
              GATEWAY_WASM_FILE =
                "${self'.packages.xc-cw-contracts}/lib/cw_xc_gateway.wasm";
            };
            enterShell = ''
              osmosisd set-env localnet
              echo 'chain-id = "osmosis-dev"' > ~/.osmosisd-local/config/client.toml 
              echo 'keyring-backend = "test"' >> ~/.osmosisd-local/config/client.toml 
              echo 'output = "json"' >> ~/.osmosisd-local/config/client.toml 
              echo 'broadcast-mode = "block"' >> ~/.osmosisd-local/config/client.toml 
              echo 'human-readable-denoms-input = false' >> ~/.osmosisd-local/config/client.toml 
              echo 'human-readable-denoms-output = false' >> ~/.osmosisd-local/config/client.toml 
              echo 'gas = ""' >> ~/.osmosisd-local/config/client.toml 
              echo 'gas-prices = ""' >> ~/.osmosisd-local/config/client.toml 
              echo 'gas-adjustment = ""' >> ~/.osmosisd-local/config/client.toml 
              echo 'fees = ""' >> ~/.osmosisd-local/config/client.toml 
              echo 'node = "${env.NODE}"' >> ~/.osmosisd-local/config/client.toml 
            '';
          }];
        };
      };
    };
}
