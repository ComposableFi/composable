{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, lib, system, subnix
    , systemCommonRust, ... }:
    let
      rust = (self.inputs.crane.mkLib pkgs).overrideToolchain
        (pkgs.rust-bin.nightly."2023-10-05".default.override {
          targets = [ "wasm32-unknown-unknown" ];
        });

      buildPolkadotNode = { name, repo, owner, rev, hash, outputHashes, }:
        rust.buildPackage (systemCommonRust.common-attrs // rec {
          inherit name;
          pname = "polkadot";
          src = pkgs.fetchgit {
            url = "https://github.com/${owner}/${repo}.git";
            inherit rev;
            sha256 = hash;
            fetchSubmodules = false;
          };
          # unfortunately any rust in nativeBuildInputs overrides rust used in buildPackage, so we fore it to be compatible with polkadot
          nativeBuildInputs = lib.remove self'.packages.rust-nightly
            systemCommonRust.common-attrs.nativeBuildInputs;
          meta = { mainProgram = "polkadot"; };
        });
      cargo-lock = builtins.fromTOML (builtins.readFile ../../code/Cargo.lock);
      rococo-runtime-dep = builtins.head
        (builtins.filter (x: x.name == "rococo-runtime") (cargo-lock.package));
      rococo-runtime-commit =
        builtins.elemAt (builtins.split "#" rococo-runtime-dep.source) 2;

      mkRelayRuntime = url: hash:
        pkgs.stdenv.mkDerivation {
          name = "relay-runtime";
          dontUnpack = true;
          src = pkgs.fetchurl {
            inherit url;
            inherit hash;
          };
          installPhase = ''
            mkdir -p $out/lib
            cp $src $out/lib/relay_runtime.compact.compressed.wasm
          '';
        };

    in {
      packages = {
        rococo-runtime-from-dep = mkRelayRuntime
          "https://github.com/paritytech/polkadot/releases/download/v0.9.38-rc12/rococo_runtime-v9381.compact.compressed.wasm"
          "sha256-Qh8oa+Y7LbGvXBXdHFarC81QGARsydvjzlPvOiNK+Xw=";
        kusama-runtime-on-parity = mkRelayRuntime
          "https://github.com/paritytech/polkadot/releases/download/v0.9.38-rc12/kusama_runtime-v9381.compact.compressed.wasm"
          "sha256-LTKYGGMyQj+hzpp+9DvpPExzwiukRHQBA+e4DDAjBto=";
        polkadot-runtime-on-parity = mkRelayRuntime
          "https://github.com/paritytech/polkadot/releases/download/v0.9.37/polkadot_runtime-v9370.compact.compressed.wasm"
          "sha256-n8+2GpqqU/kHderUqea4Q7yv4UmsESw25laH1/oZryE=";

        polkadot-node-on-parity-westend = buildPolkadotNode rec {
          name = "polkadot-node-on-parity-westend";
          repo = "polkadot";
          owner = "paritytech";
          rev = "e203bfb396ed949f102720debf32fb98166787af";
          hash = "sha256-+rGrAyQH//m6xFiUstDiZKhvHq928rs36TajT/QxrKM=";
          outputHashes = {
            "sub-tokens-0.1.0" =
              "sha256-GvhgZhOIX39zF+TbQWtTCgahDec4lQjH+NqamLFLUxM=";
            "binary-merkle-tree-4.0.0-dev" =
              "sha256-ngtW11MGs+fcuCp9J5NH+dYJeK4YM5vWpRk0OuLYHus=";
          };
        };
      };
    };
}
