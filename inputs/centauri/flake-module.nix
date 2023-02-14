{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, ... }:
    let
      centauri-src = pkgs.fetchFromGitHub {
        owner = "ComposableFi";
        repo = "centauri";
        rev = "94bf87a44694b04917a7ab735487c8f87a64737d";
        hash = "sha256-tG4WLAUtQ2iaaS4t/Condj6B1FDa/5VDoRwyBsJDfr4=";
      };
    in {
      packages = rec {
        centauri-codegen = crane.stable.buildPackage {
          name = "centauri-codegen";
          cargoArtifacts = crane.stable.buildDepsOnly {
            src = centauri-src;
            doCheck = false;
            cargoExtraArgs = "-p codegen";
            cargoTestCommand = "";
          };
          src = centauri-src;
          doCheck = false;
          cargoExtraArgs = "-p codegen";
          cargoTestCommand = "";
          meta = { mainProgram = "codegen"; };
        };

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

        hyperspace-dali-image = pkgs.dockerTools.buildImage {
          tag = "latest";
          name = "hyperspace-dali";
          config = { Entrypoint = [ "${hyperspace-dali}/bin/hyperspace" ]; };
        };

        hyperspace-dali = let
          src = pkgs.stdenv.mkDerivation rec {
            name = "centauri";
            pname = "${name}";
            buildInputs = [ self'.packages.dali-subxt-client ];
            src = centauri-src;
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
            src = centauri-src;
            doCheck = false;
            cargoExtraArgs = "-p hyperspace --features dali";
            cargoTestCommand = "";
            BuildInputs = [ pkgs.protobuf ];
            PROTOC = "${pkgs.protobuf}/bin/protoc";
            PROTOC_INCLUDE = "${pkgs.protobuf}/include";
            PROTOC_NO_VENDOR = "1";
          };
          src = centauri-src;
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
