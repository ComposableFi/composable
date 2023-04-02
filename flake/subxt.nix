{ ... }: {
  perSystem = { self', pkgs, systemCommonRust, ... }: {
    packages = let
      mkSubxtClient = name: runtime:
        pkgs.stdenv.mkDerivation {
          inherit name;
          dontUnpack = true;
          buildInputs = [
            self'.packages.centauri-codegen
            runtime
            self'.packages.rococo-wasm-runtime-current
          ];
          nativeBuildInputs = with pkgs;
            [ clang ] ++ systemCommonRust.darwin-deps;

          installPhase = ''
            mkdir --parents $out
            ${pkgs.lib.meta.getExe self'.packages.centauri-codegen} \
              --path $out \
              --parachain-wasm=${runtime}/lib/runtime.optimized.wasm \
              --relaychain-wasm=${self'.packages.rococo-wasm-runtime-current}/lib/rococo_runtime.compact.compressed.wasm
          '';
        };
    in {
      composable-rococo-subxt-client =
        mkSubxtClient "composable-rococo-subxt-client"
        self'.packages.composable-runtime;
      picasso-rococo-subxt-client = mkSubxtClient "picasso-rococo-subxt-client"
        self'.packages.picasso-runtime;
    };
  };
}
