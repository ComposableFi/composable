{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, lib, system, subnix
    , systemCommonRust, ... }:
    let
      rust = (self.inputs.crane.mkLib pkgs).overrideToolchain
        (pkgs.rust-bin.nightly."2023-06-15".default.override {
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

        rococo-runtime-on-parity = mkRelayRuntime
          "https://github.com/paritytech/polkadot/releases/download/v0.9.39/rococo_runtime-v9390.compact.compressed.wasm"
          "sha256-eUK9jF8gXbYVtynCXevpJixBBN2gQEnVfyOp3kwTrt8=";
        kusama-runtime-on-parity = mkRelayRuntime
          "https://github.com/paritytech/polkadot/releases/download/v0.9.38-rc12/kusama_runtime-v9381.compact.compressed.wasm"
          "sha256-LTKYGGMyQj+hzpp+9DvpPExzwiukRHQBA+e4DDAjBto=";

        westend-runtime-on-parity = mkRelayRuntime
          "https://github.com/paritytech/polkadot/releases/download/v0.9.40/westend_runtime-v9401.compact.compressed.wasm"
          "sha256-9FbZ5moShA+0VB54cwGebiYuKLNSruWYSwJ0gDOHbCU=";
        polkadot-runtime-on-parity = mkRelayRuntime
          "https://github.com/paritytech/polkadot/releases/download/v0.9.37/polkadot_runtime-v9370.compact.compressed.wasm"
          "sha256-n8+2GpqqU/kHderUqea4Q7yv4UmsESw25laH1/oZryE=";

        polkadot-node-from-dep = buildPolkadotNode rec {
          name = "polkadot-node-from-dep";
          repo = "polkadot";
          owner = "paritytech";
          rev = rococo-runtime-commit;
          hash = "sha256-KYmMMcQMkkXfWj5ZTr549a/8ftELKo0PUvCrmRMiDaE=";
          outputHashes = {
            "sub-tokens-0.1.0" =
              "sha256-GvhgZhOIX39zF+TbQWtTCgahDec4lQjH+NqamLFLUxM=";
            "binary-merkle-tree-4.0.0-dev" =
              "sha256-/8bGqnM/yqtCgVWkIaVEySZSV3XGYuiA3JuyHYTp2lw=";
          };
        };

        polkadot-node-on-parity-rococo = buildPolkadotNode rec {
          name = "polkadot-node-on-parity-rococo";
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

        polkadot-node-on-parity-kusama = buildPolkadotNode rec {
          name = "polkadot-node-on-parity-kusama";
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

        polkadot-node-on-parity-polkadot = buildPolkadotNode rec {
          name = "polkadot-node-on-parity-polkadot";
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

        polkadot-live-runtime-node = buildPolkadotNode rec {
          name = "polkadot-live-runtime-node";
          repo = "polkadot";
          owner = "paritytech";
          rev = "52209dcfe546ff39cc031b92d64e787e7e8264d4";
          hash = "sha256-927W8su86sPRyCF9eijm58X2uPBPnsR4KgJTIxVIcqA=";
          outputHashes = {
            "ark-secret-scalar-0.0.2" =
              "sha256-EUxl9ooQja1RJFBC7uxDwe/AcQSepclnXCs1U5EtDOs=";
            "common-0.1.0" =
              "sha256-3OKBPpk0exdlV0N9rJRVIncSrkwdI8bkYL2QNsJl+sY=";
            "fflonk-0.1.0" =
              "sha256-MNvlePHQdY8DiOq6w7Hc1pgn7G58GDTeghCKHJdUy7E=";
            "binary-merkle-tree-4.0.0-dev" =
              "sha256-GJGCvJnJr6ZsLkEgi62zAJyMkGITj1/mW0/BjJQXe8U=";
            "sub-tokens-0.1.0" =
              "sha256-GvhgZhOIX39zF+TbQWtTCgahDec4lQjH+NqamLFLUxM=";
          };
        };
      };
    };
}
