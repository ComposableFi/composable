{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, ... }:

    {
      packages = rec {
        hyperspace-config = pkgs.writeText "config.toml" ''
          [chain_a]
          type = "parachain"
          name = "picasso_1"
          para_id = 2087
          parachain_rpc_url = "ws://devnet-a:9988"
          relay_chain_rpc_url = "ws://devnet-a:9944"
          channel_whitelist = []
          commitment_prefix = "0x6962632f"
          private_key = "//Alice"
          ss58_version = 49
          finality_protocol = "Grandpa"
          key_type = "sr25519"

          [chain_b]
          type = "parachain"
          name = "picasso_2"
          para_id = 2087
          parachain_rpc_url = "ws://devnet-b:29988"
          relay_chain_rpc_url = "ws://devnet-b:29944"
          channel_whitelist = []
          commitment_prefix = "0x6962632f"
          private_key = "//Alice"
          ss58_version = 49
          finality_protocol = "Grandpa"
          key_type = "sr25519"

          [core]
          prometheus_endpoint = "https://127.0.0.1"
        '';

        hyperspace-dali-container = pkgs.dockerTools.buildImage {
          tag = "latest";
          name = "hyperspace-dali";
          config = { Entrypoint = [ "${hyperspace-dali}/bin/hyperspace" ]; };
        };

        hyperspace-dali = let
          src = pkgs.stdenv.mkDerivation rec {
            name = "centauri";
            pname = "${name}";
            buildInputs = [ self'.packages.dali-subxt-client ];
            src = pkgs.fetchFromGitHub {
              owner = "ComposableFi";
              repo = "centauri";
              rev = "9b3d6db98d7fc32d716cb851e9ac7e3c197a7b54";
              hash = "sha256-OCeJkTcINQV21du/56gxzq5MG9yFY1TM+6d2Y3/zsBE=";
            };
            patchPhase = "";
            installPhase = ''
              mkdir $out
              cp --archive $src/. $out/
              chmod u+w $out/utils/subxt/generated/src/{parachain.rs,relaychain.rs}
              cp ${self'.packages.dali-subxt-client}/* $out/utils/subxt/generated/src/
            '';
            dontFixup = true;
            dontStrip = true;
          };
        in crane.stable.buildPackage rec {
          name = "hyperspace-dali";
          pname = "${name}";
          cargoArtifacts = crane.stable.buildDepsOnly {
            inherit src;
            doCheck = false;
            cargoExtraArgs = "-p hyperspace --features dali";
            cargoTestCommand = "";
            BuildInputs = [ pkgs.protobuf ];
            PROTOC = "${pkgs.protobuf}/bin/protoc";
            PROTOC_INCLUDE = "${pkgs.protobuf}/include";
            PROTOC_NO_VENDOR = "1";
          };
          inherit src;
          BuildInputs = [ pkgs.protobuf ];
          PROTOC = "${pkgs.protobuf}/bin/protoc";
          PROTOC_INCLUDE = "${pkgs.protobuf}/include";
          PROTOC_NO_VENDOR = "1";
          doCheck = false;
          cargoExtraArgs = "-p hyperspace --features dali";
          cargoTestCommand = "";
          meta = { mainProgram = "hyperspace"; };
        };
      };
    };
}
