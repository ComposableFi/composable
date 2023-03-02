{ ... }: {
  perSystem = { self', pkgs, ... }: {
    packages = {
      dali-subxt-client = pkgs.stdenv.mkDerivation {
        name = "dali-subxt-client";
        dontUnpack = true;
        buildInputs = [
          self'.packages.centauri-codegen
          self'.packages.dali-runtime
          self'.packages.rococo-wasm-runtime
        ];
        installPhase = ''
          mkdir --parents $out
          ${pkgs.lib.meta.getExe self'.packages.centauri-codegen} \
            --path $out \
            --parachain-wasm=${self'.packages.dali-runtime}/lib/runtime.optimized.wasm \
            --relaychain-wasm=${self'.packages.rococo-wasm-runtime}/lib/rococo_runtime.compact.compressed.wasm
        '';
      };
    };
  };
}
