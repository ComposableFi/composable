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
          osmosisd
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
            ${centauri-mainnet-shell}
            ${osmosis-mainnet-shell}
            ${neutron-mainnet-shell}
          '';

        }];
      };

      centauri-mainnet-shell = ''
        rm --force --recursive ~/.banksy
        mkdir --parents ~/.banksy/config
        echo 'keyring-backend = "os"' >> ~/.banksy/config/client.toml
        echo 'output = "json"' >> ~/.banksy/config/client.toml
        echo 'node = "${networks.pica.mainnet.NODE}"' >> ~/.banksy/config/client.toml
        echo 'chain-id = "centauri-1"' >> ~/.banksy/config/client.toml
      '';
      neutron-mainnet-shell = ''
        rm --force --recursive ~/.neutrond
        mkdir --parents ~/.neutrond/config
        CLIENT_CONFIG=~/.neutrond/config/client.toml
        if [[ ! -f $CLIENT_CONFIG ]]; then
          echo 'keyring-backend = "os"' > $CLIENT_CONFIG
        fi
        dasel put --type=string --write=toml --file="$CLIENT_CONFIG" --value  "os" "keyring-backend"
        dasel put --type=string --write=toml --file="$CLIENT_CONFIG" --value "json" "output" 
        dasel put --type=string --write=toml --file="$CLIENT_CONFIG" --value "${networks.neutron.mainnet.NODE}" "node" 
        dasel put --type=string --write=toml --file="$CLIENT_CONFIG" --value  "${networks.neutron.mainnet.CHAIN_ID}" "chain-id"
        dasel put --type=string --write=toml --file="$CLIENT_CONFIG" --value  "sync" "broadcast-mode"
      '';
      osmosis-mainnet-shell = ''
        rm ~/.osmosisd/config/client.toml 
        osmosisd set-env mainnet      
      '';
      contracts-env = {
        EXECUTOR_WASM_FILE = "${
            self.inputs.cvm.packages."${system}".cw-cvm-executor
          }/lib/cw_cvm_executor.wasm";
        OUTPOST_WASM_FILE = "${
            self.inputs.cvm.packages."${system}".cw-cvm-outpost
          }/lib/cw_cvm_outpost.wasm";
        ORDER_WASM_FILE = "${
            self.inputs.cvm.packages."${system}".cw-mantis-order
          }/lib/cw_mantis_order.wasm";
      };
    in {
      packages = {
        all-deps-shell = pkgs.linkFarmFromDrvs "all-deps-shell" tools;
      };
      devShells = {
        default = self.inputs.devenv.lib.mkShell all-deps-attrs;

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
                  self.inputs.cvm.packages."${system}".cw-cvm-outpost
                }/lib/cw_cvm_outpost.wasm";
            };
            enterShell = ''
              rm --force --recursive ~/.neutrond
              mkdir --parents ~/.neutrond/config
              echo 'keyring-backend = "test"' > ~/.neutrond/config/client.toml
              echo 'output = "json"' >> ~/.neutrond/config/client.toml
              echo 'node = "${env.NODE}"' >> ~/.neutrond/config/client.toml
              echo 'chain-id = "${env.CHAIN_ID}"' >> ~/.neutrond/config/client.toml               
              echo ${networks.devnet.mnemonics.APPLICATION1} | "$BINARY" keys add APPLICATION1 --recover --keyring-backend=test --output=json            
            '';
          }];
        };

        centauri-devnet = self.inputs.devenv.lib.mkShell {
          inherit pkgs;
          inputs = self.inputs;
          modules = [rec {
            packages = [ self'.packages.centaurid ];
            env = networks.pica.devnet // {
              EXECUTOR_WASM_FILE = "${
                  self.inputs.cvm.packages."${system}".cw-cvm-executor
                }/lib/cw_cvm_executor.wasm";
              OUTPOST_WASM_FILE = "${
                  self.inputs.cvm.packages."${system}".cw-cvm-outpost
                }/lib/cw_cvm_outpost.wasm";
            };
            enterShell = ''
              mkdir --parents "$CONFIG_FOLDER"
              dasel put --type=string --write=toml --file="$CONFIG_FOLDER"/client.toml --value  "test" "keyring-backend"
              dasel put --type=string --write=toml --file="$CONFIG_FOLDER"/client.toml --value "json" "output" 
              dasel put --type=string --write=toml --file="$CONFIG_FOLDER"/client.toml --value "${env.NODE}" "node" 
              dasel put --type=string --write=toml --file="$CONFIG_FOLDER"/client.toml --value  "${env.CHAIN_ID}" "chain-id"
              dasel put --type=string --write=toml --file="$CONFIG_FOLDER"/client.toml --value  "sync" "broadcast-mode"
              ${bashTools.export networks.devnet.mnemonics}

              echo "$APPLICATION1" | "$BINARY" keys add APPLICATION1 --recover 2>>/dev/null
              echo "$DEMO_MNEMONIC_1" | "$BINARY" keys add DEMO_MNEMONIC_1 --recover 2>/dev/null
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
              BINARY = "centaurid";
              NODE = "https://rpc-t.composable.nodestake.top:443";
            } // contracts-env;
          }];
        };

        centauri-mainnet = self.inputs.devenv.lib.mkShell {
          inherit pkgs;
          inputs = self.inputs;
          modules = [{
            packages = [ self'.packages.centaurid ];
            env = networks.pica.mainnet // contracts-env;
            enterShell = centauri-mainnet-shell;
          }];
        };

        all-mainnet = self.inputs.devenv.lib.mkShell {
          inherit pkgs;
          inputs = self.inputs;
          modules = [{
            packages = with self'.packages; [ neutrond osmosisd centaurid ];
            env = contracts-env;

            enterShell = ''
              ${centauri-mainnet-shell}

              ${osmosis-mainnet-shell}

              ${neutron-mainnet-shell}
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
                  self.inputs.cvm.packages."${system}".cw-cvm-outpost
                }/lib/cw_cvm_outpost.wasm";
            };

            enterShell = neutron-mainnet-shell;
          }];
        };

        osmosis-mainnet = self.inputs.devenv.lib.mkShell {
          inherit pkgs;
          inputs = self.inputs;
          modules = [{
            packages = [ self'.packages.osmosisd ];
            env = networks.osmosis.mainnet // contracts-env;
            enterShell = osmosis-mainnet-shell;
          }];
        };

        osmosis-testnet = self.inputs.devenv.lib.mkShell {
          inherit pkgs;
          inputs = self.inputs;
          modules = [{
            packages = [ self'.packages.osmosisd ];
            env = osmosis.env.testnet // contracts-env;
          }];
        };

        osmosis-devnet = self.inputs.devenv.lib.mkShell {
          inherit pkgs;
          inputs = self.inputs;
          modules = [rec {
            packages = [ self'.packages.osmosisd ];
            env = networks.osmosis.devnet // contracts-env;
            enterShell = ''
              dasel put --type=string --write=toml --file="$CONFIG_FOLDER"/client.toml --value  "test" "keyring-backend"
              dasel put --type=string --write=toml --file="$CONFIG_FOLDER"/client.toml --value "json" "output" 
              dasel put --type=string --write=toml --file="$CONFIG_FOLDER"/client.toml --value "${env.NODE}" "node" 
              dasel put --type=string --write=toml --file="$CONFIG_FOLDER"/client.toml --value  "${env.CHAIN_ID}" "chain-id"
              dasel put --type=string --write=toml --file="$CONFIG_FOLDER"/client.toml --value  "sync" "broadcast-mode"

              ${bashTools.export networks.devnet.mnemonics}

              echo "$APPLICATION1" | "$BINARY" keys add APPLICATION1 --recover 2>>/dev/null
              echo "$DEMO_MNEMONIC_1" | "$BINARY" keys add DEMO_MNEMONIC_1 --recover 2>/dev/null            
            '';
          }];
        };

        osmosis-remote-devnet = self.inputs.devenv.lib.mkShell {
          inherit pkgs;
          inputs = self.inputs;
          modules = [rec {
            packages = [ self'.packages.osmosisd ];
            env = osmosis.env.remote-devnet // contracts-env;
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
