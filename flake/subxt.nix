{ ... }: {
  perSystem = { self', pkgs, ... }: {
    packages =
      let
        mkRelayAndParaSubxtClient = relay: para: pkgs.stdenv.mkDerivation {
          name = "subxt-client";
          dontUnpack = true;
          buildInputs = [
            self'.packages.centauri-codegen
            para
            relay
          ];
          installPhase = ''
            mkdir --parents $out
            ${pkgs.lib.meta.getExe self'.packages.centauri-codegen} \
              --path $out \
              --parachain-wasm=${para}/lib/runtime.optimized.wasm \
              --relaychain-wasm=${relay}/lib/rococo_runtime.compact.compressed.wasm
          '';
        };
      in
      {
        dali-subxt-client = mkRelayAndParaSubxtClient self'.packages.rococo-wasm-runtime self'.packages.dali-runtime;
        picasso-subxt-client = mkRelayAndParaSubxtClient self'.packages.rococo-wasm-runtime self'.packages.picasso-runtime;
        composable-subxt-client = mkRelayAndParaSubxtClient self'.packages.rococo-wasm-runtime self'.packages.composable-runtime;
      };
  };
}
