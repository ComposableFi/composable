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
              centaurid
              osmosisd
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

        neutron-devnet = self.inputs.devenv.lib.mkShell {
          inherit pkgs;
          inputs = self.inputs;
          modules = [rec {
            packages = [ self'.packages.neutrond ];
            env = networks.neutron.devnet // {
              DIR = "devnet/.neutrond";
              EXECUTOR_WASM_FILE = "${
                  self.inputs.cvm.packages."${system}".cw-cvm-executor
                }/lib/cw_cvm_executor.wasm";
              OUTPOST_WASM_FILE = "${
                  self.inputs.cvm.packages."${system}".cw-cvm-gateway
                }/lib/cw_cvm_gateway.wasm";
            };
            enterShell = ''
              rm --force --recursive ~/.neutrond
              mkdir --parents ~/.neutrond/config
              echo 'keyring-backend = "test"' >> ~/.neutrond/config/client.toml
              echo 'output = "json"' >> ~/.neutrond/config/client.toml
              echo 'node = "${env.NODE}"' >> ~/.neutrond/config/client.toml
              echo 'chain-id = "${env.CHAIN_ID}"' >> ~/.neutrond/config/client.toml               
              echo ${networks.devnet.mnemonics.APPLICATION1} | "$BINARY" keys add APPLICATION1 --recover --keyring-backend test --output json            
            '';
          }];
        };

        centauri-devnet = self.inputs.devenv.lib.mkShell {
          inherit pkgs;
          inputs = self.inputs;
          modules = [rec {
            packages = [ self'.packages.centaurid ];
            env = networks.pica.devnet // {
              DIR = "devnet/.centaurid";
              NODE = "tcp://localhost:26657";
              EXECUTOR_WASM_FILE = "${
                  self.inputs.cvm.packages."${system}".cw-cvm-executor
                }/lib/cw_cvm_executor.wasm";
              OUTPOST_WASM_FILE = "${
                  self.inputs.cvm.packages."${system}".cw-cvm-gateway
                }/lib/cw_cvm_gateway.wasm";
            };
            enterShell = ''
              rm --force --recursive ~/.centauri
              mkdir --parents ~/.centauri/config
              echo 'keyring-backend = "test"' >> ~/.centauri/config/client.toml
              echo 'output = "json"' >> ~/.centauri/config/client.toml
              echo 'node = "${env.NODE}"' >> ~/.centauri/config/client.toml
              echo 'chain-id = "${env.CHAIN_ID}"' >> ~/.centauri/config/client.toml               
              ${bashTools.export networks.devnet.mnemonics}
              echo "$APPLICATION1" | "$BINARY" keys add APPLICATION1 --recover --keyring-backend test --output json            
              echo "$DEMO_MNEMONIC_1" | "$BINARY" keys add DEMO_MNEMONIC_1 --recover --keyring-backend test --output json
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
              EXECUTOR_WASM_FILE = "${
                  self.inputs.cvm.packages."${system}".cw-cvm-executor
                }/lib/cw_cvm_executor.wasm";
              OUTPOST_WASM_FILE = "${
                  self.inputs.cvm.packages."${system}".cw-cvm-gateway
                }/lib/cw_cvm_gateway.wasm";
            };
          }];
        };

        centauri-mainnet = self.inputs.devenv.lib.mkShell {
          inherit pkgs;
          inputs = self.inputs;
          modules = [{
            packages = [ self'.packages.centaurid ];
            env = networks.pica.mainnet // {
              EXECUTOR_WASM_FILE = "${
                  self.inputs.cvm.packages."${system}".cw-cvm-executor
                }/lib/cw_cvm_executor.wasm";
              OUTPOST_WASM_FILE = "${
                  self.inputs.cvm.packages."${system}".cw-cvm-gateway
                }/lib/cw_cvm_gateway.wasm";
              ORDER_WASM_FILE = "${
                  self.inputs.cvm.packages."${system}".cw-mantis-order
                }/lib/cw_mantis_order.wasm";
            };

            enterShell = ''
              rm --force --recursive ~/.banksy
              mkdir --parents ~/.banksy/config
              echo 'keyring-backend = "os"' >> ~/.banksy/config/client.toml
              echo 'output = "json"' >> ~/.banksy/config/client.toml
              echo 'node = "${networks.pica.mainnet.NODE}"' >> ~/.banksy/config/client.toml
              echo 'chain-id = "centauri-1"' >> ~/.banksy/config/client.toml
            '';
          }];
        };

        neutron-mainnet = self.inputs.devenv.lib.mkShell {
          inherit pkgs;
          inputs = self.inputs;
          modules = [{
            packages = [ self'.packages.neutrond ];
            env = networks.neutron.mainnet // {
              EXECUTOR_WASM_FILE = "${
                  self.inputs.cvm.packages."${system}".cw-cvm-executor
                }/lib/cw_cvm_executor.wasm";
              OUTPOST_WASM_FILE = "${
                  self.inputs.cvm.packages."${system}".cw-cvm-gateway
                }/lib/cw_cvm_gateway.wasm";
            };

            enterShell = ''
              rm --force --recursive ~/.neutron
              mkdir --parents ~/.neutron/config
              echo 'keyring-backend = "os"' >> ~/.neutron/config/client.toml
              echo 'output = "json"' >> ~/.neutron/config/client.toml
              echo 'node = "https://rpc-kralum.neutron-1.neutron.org:443"' >> ~/.neutron/config/client.toml
              echo 'chain-id = "${networks.neutron.mainnet.CHAIN_ID}"' >> ~/.neutron/config/client.toml
            '';
          }];
        };

        osmosis-mainnet = self.inputs.devenv.lib.mkShell {
          inherit pkgs;
          inputs = self.inputs;
          modules = [{
            packages = [ self'.packages.osmosisd ];
            env = networks.osmosis.mainnet // {
              EXECUTOR_WASM_FILE = "${
                  self.inputs.cvm.packages."${system}".cw-cvm-executor
                }/lib/cw_cvm_executor.wasm";
              OUTPOST_WASM_FILE = "${
                  self.inputs.cvm.packages."${system}".cw-cvm-gateway
                }/lib/cw_cvm_gateway.wasm";
              ORDER_WASM_FILE = "${
                  self.inputs.cvm.packages."${system}".cw-mantis-order
                }/lib/cw_mantis_order.wasm";
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
              EXECUTOR_WASM_FILE = "${
                  self.inputs.cvm.packages."${system}".cw-cvm-executor
                }/lib/cw_cvm_executor.wasm";
              OUTPOST_WASM_FILE = "${
                  self.inputs.cvm.packages."${system}".cw-cvm-gateway
                }/lib/cw_cvm_gateway.wasm";
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
              EXECUTOR_WASM_FILE = "${
                  self.inputs.cvm.packages."${system}".cw-cvm-executor
                }/lib/cw_cvm_executor.wasm";
              OUTPOST_WASM_FILE = "${
                  self.inputs.cvm.packages."${system}".cw-cvm-gateway
                }/lib/cw_cvm_gateway.wasm";
              NODE = "tcp://localhost:${
                  builtins.toString networks.osmosis.devnet.PORT
                }";
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
              EXECUTOR_WASM_FILE = "${
                  self.inputs.cvm.packages."${system}".cw-cvm-executor
                }/lib/cw_cvm_executor.wasm";
              OUTPOST_WASM_FILE = "${
                  self.inputs.cvm.packages."${system}".cw-cvm-gateway
                }/lib/cw_cvm_gateway.wasm";
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
