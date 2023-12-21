{ ... }: {
  perSystem = { self', pkgs, systemCommonRust, subnix, ... }: {
    packages = let
      mkSubxtClient = name: parachain-runtime: relay-runtime:
        pkgs.stdenv.mkDerivation (subnix.subenv // {
          inherit name;
          dontUnpack = true;
          buildInputs = with self'.packages; with pkgs; [ subwasm subxt ];

          installPhase = ''
            mkdir --parents $out/lib
            subwasm metadata ${parachain-runtime}/lib/runtime.optimized.wasm --format scale > $out/lib/parachain.scale
            subwasm metadata ${relay-runtime}/lib/relay_runtime.compact.compressed.wasm --format scale > $out/lib/relaychain.scale
            subxt codegen --file $out/lib/parachain.scale > $out/parachain.rs
            subxt codegen --file $out/lib/relaychain.scale > $out/relaychain.rs
          '';
        });
    in {
      subxt-codegen-picasso = pkgs.writeShellApplication {
        name = "subxt-codegen-picasso";
        runtimeInputs = [ pkgs.git pkgs.yq ];
        text = ''
          subxt codegen --url=wss://rpc.composable.finance:443  > $1
        '';
      };

      subxt-codegen-composable = pkgs.writeShellApplication {
        name = "subxt-codegen-composable";
        runtimeInputs = [ pkgs.git pkgs.yq ];
        text = ''
          subxt codegen --url=wss://picasso-rpc.composable.finance:443 > $1
        '';
      };
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
