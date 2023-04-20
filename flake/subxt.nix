{ ... }: {
  perSystem = { self', pkgs, systemCommonRust, subnix, ... }: {
    packages = let
      mkSubxtClient = name: parachain-runtime: relay-runtime:
        pkgs.stdenv.mkDerivation (subnix.subenv // {
          inherit name;
          dontUnpack = true;
          buildInputs =
            [ self'.packages.centauri-codegen parachain-runtime relay-runtime ];

          installPhase = ''
            mkdir --parents $out
            ${pkgs.lib.meta.getExe self'.packages.centauri-codegen} \
              --path $out \
              --parachain-wasm=${parachain-runtime}/lib/runtime.optimized.wasm \
              --relaychain-wasm=${relay-runtime}/lib/relay_runtime.compact.compressed.wasm
          '';
        });
    in {
      composable-rococo-subxt-client =
        mkSubxtClient "composable-rococo-subxt-client"
        self'.packages.composable-runtime
        self'.packages.rococo-runtime-from-dep;

      composable-polkadot-subxt-client =
        mkSubxtClient "composable-polkadot-subxt-client"
        self'.packages.composable-runtime
        self'.packages.polkadot-runtime-on-parity;

      picasso-rococo-subxt-client = mkSubxtClient "picasso-rococo-subxt-client"
        self'.packages.picasso-runtime self'.packages.rococo-runtime-from-dep;

      picasso-kusama-subxt-client = mkSubxtClient "picasso-rococo-subxt-client"
        self'.packages.picasso-runtime self'.packages.kusama-runtime-on-parity;
    };
  };
}
