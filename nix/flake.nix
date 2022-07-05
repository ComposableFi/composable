{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, utils, rust-overlay }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        rust-nightly = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
          toolchain.default.override {
            extensions = [ "rust-src" ];
            targets = [ "wasm32-unknown-unknown" ];
          });
        rust-stable = pkgs.rust-bin.stable.latest.default;
      in with pkgs; rec {
        packages.composable = rustPlatform.buildRustPackage rec {
          pname = "composable";
          version = "2.1.6";
          src = let
            customFilter = name: type:
              !(type == "directory" && baseNameOf name == "nix");
          in lib.cleanSourceWith {
            filter = lib.cleanSourceFilter;
            src = lib.cleanSourceWith {
              filter =
                nix-gitignore.gitignoreFilterPure customFilter [ ../.gitignore ]
                ../.;
              src = ../.;
            };
          };
          nativeBuildInputs = [ rust-nightly clang ];
          cargoLock = {
            lockFile = ../Cargo.lock;
            outputHashes = {
              "parity-scale-codec-3.1.2" =
                "sha256-GVonCdlCgrU/GVOL750BBeJBOdNVbUNcDKkigO+Sc/8=";
              "beefy-gadget-4.0.0-dev" =
                "sha256-gm6vrTE9iIcuQg+uHqR2I44nsex+92Bkww9EpV22iAQ=";
              "beefy-generic-client-0.1.0" =
                "sha256-gIsymJQk2AsAj/S/ewWbTYRGx49ySB+k24879sc1YhA=";
              "bp-header-chain-0.1.0" =
                "sha256-HIgHz/HLLt6qVrSJ/9EsTDRqz8VbAuwaqoSgWiZ3HTg=";
              "cumulus-client-cli-0.1.0" =
                "sha256-cOBGm6rymkx+V5r/6Hdn6KVzxyFQxqFVsHuctFsUMB4=";
              "ibc-0.15.0" =
                "sha256-qdxdaJJPhdujo6k3IYYSRBT60Rpc/hu8hcVnY6xpO3I=";
              "ics23-0.8.0-alpha" =
                "sha256-+bNdLpM/ILn8ia+jCtyMZsMgjAjJQm8Tmwe3YE7k1HA=";
              "orml-rewards-0.4.1-dev" =
                "sha256-nZTDF38Qz2jEI8lEqJS3aTCXuM7dlzQ6W9dZ6+Sf/ZA";
              "simnode-runtime-apis-0.1.0" =
                "sha256-9iNn8LEXRDsj8+S9I25SwwYGxAYFQ23nAlKFhh1w/lc=";
              "simple-iavl-0.1.0" =
                "sha256-Jc+CRvayJ5MGc/i5InjcRejL9h3nJTKUWcf/XSCIazw=";
              "subxt-0.21.0" =
                "sha256-DS18xHovzy3LWkC1YeW0wWkd9JOtniivSjSharPy5zQ=";
              "tendermint-0.24.0-pre.2" =
                "sha256-dPUdkIviH5Rg3pumbP+YmOc9ocDgxeeOIq9STmDeXOE=";
              "xcm-emulator-0.1.0" =
                "sha256-eGPNpuC5VKbLfRSkL0rTB4Tuww6ubIV9pcsf+p2E5EM=";
            };
          };
          CARGO_NET_OFFLINE = "1";
          SKIP_POLKADOT_RUNTIME_WASM_BUILD = "1";
          SKIP_KUSAMA_RUNTIME_WASM_BUILD = "1";
          SKIP_ROCOCO_RUNTIME_WASM_BUILD = "1";
          LD_LIBRARY_PATH = lib.strings.makeLibraryPath [
            stdenv.cc.cc.lib
            llvmPackages.libclang.lib
          ];
          LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
          PROTOC = "${protobuf}/bin/protoc";
          ROCKSDB_LIB_DIR = "${rocksdb}/lib";
        };
        defaultPackage = packages.composable;
        apps.composable = utils.lib.mkApp { drv = packages.composable; };
        defaultApp = apps.composable;
        devShell = mkShell { nativeBuildInputs = [ rust-stable ]; };
      });
}
