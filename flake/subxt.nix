{ ... }: {
  perSystem = { self', pkgs, systemCommonRust, subnix, ... }: {
    packages = let
      mkSubxtClient = name: runtime:
        pkgs.stdenv.mkDerivation (subnix.subenv // {
          inherit name;
          dontUnpack = true;
          buildInputs = [
            self'.packages.centauri-codegen
            runtime
            self'.packages.rococo-runtime-from-dep
          ];

          installPhase = ''
            mkdir --parents $out
            ${pkgs.lib.meta.getExe self'.packages.centauri-codegen} \
              --path $out \
              --parachain-wasm=${runtime}/lib/runtime.optimized.wasm \
              --relaychain-wasm=${self'.packages.rococo-runtime-from-dep}/lib/relay_runtime.compact.compressed.wasm
          '';
        });
    in {
      composable-rococo-subxt-client =
        mkSubxtClient "composable-rococo-subxt-client"
        self'.packages.composable-runtime;
      picasso-rococo-subxt-client = mkSubxtClient "picasso-rococo-subxt-client"
        self'.packages.picasso-runtime;
    };
  };
}
