{ ... }: {
  perSystem = { self', pkgs, ... }: {
    packages = {
      composable-subxt-client = pkgs.stdenv.mkDerivation {
        name = "composable-subxt-client";
        dontUnpack = true;
        buildInputs = [
          self'.packages.centauri-codegen
          self'.packages.composable-runtime
          self'.packages.rococo-wasm-runtime
          pkgs.rustfmt
        ];
        installPhase = ''
          mkdir -p $out

          ${pkgs.lib.meta.getExe self'.packages.centauri-codegen} \
            --path $out \
            --parachain-wasm=${self'.packages.composable-runtime}/lib/runtime.optimized.wasm \
            --relaychain-wasm=${self'.packages.rococo-wasm-runtime}/lib/rococo_runtime.compact.compressed.wasm

          rustfmt --edition=2018 $out/*
        '';
      };

      picasso-subxt-client = pkgs.stdenv.mkDerivation {
        name = "picasso-subxt-client";
        dontUnpack = true;
        buildInputs = [
          self'.packages.centauri-codegen
          self'.packages.picasso-runtime
          self'.packages.rococo-wasm-runtime
          pkgs.rustfmt
        ];
        installPhase = ''
          mkdir -p $out

          ${pkgs.lib.meta.getExe self'.packages.centauri-codegen} \
            --path $out \
            --parachain-wasm=${self'.packages.picasso-runtime}/lib/runtime.optimized.wasm \
            --relaychain-wasm=${self'.packages.rococo-wasm-runtime}/lib/rococo_runtime.compact.compressed.wasm

          rustfmt --edition=2018 $out/*
        '';
      };

      dali-subxt-client = pkgs.stdenv.mkDerivation {
        name = "dali-subxt-client";
        dontUnpack = true;
        buildInputs = [
          self'.packages.centauri-codegen
          self'.packages.dali-runtime
          self'.packages.rococo-wasm-runtime
          pkgs.rustfmt
        ];
        installPhase = ''
          mkdir -p $out

          ${pkgs.lib.meta.getExe self'.packages.centauri-codegen} \
            --path $out \
            --parachain-wasm=${self'.packages.dali-runtime}/lib/runtime.optimized.wasm \
            --relaychain-wasm=${self'.packages.rococo-wasm-runtime}/lib/rococo_runtime.compact.compressed.wasm

          rustfmt --edition=2018 $out/*
        '';
      };
    };
  };
}
