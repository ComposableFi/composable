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
          cargoExtraArgs = "--features='fast-runtime'";
          # unfortunately any rust in nativeBuildInputs overrides rust used in buildPackage, so we fore it to be compatible with polkadot
          nativeBuildInputs = lib.remove self'.packages.rust-nightly
            systemCommonRust.common-attrs.nativeBuildInputs;
          meta = { mainProgram = "polkadot"; };
        });

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

        polkadot-fast-runtime = buildPolkadotNode rec {
          name = "polkadot-fast-runtime";
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
