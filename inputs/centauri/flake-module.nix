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

        dali-subxt-patch = pkgs.stdenv.mkDerivation rec {
          name = "dali-subxt-patch";
          pname = "${name}";
          buildInputs = [ self'.packages.dali-subxt-client ];
          src = centauri-src;
          patchPhase = "true";
          installPhase = ''
            mkdir --parents $out
            set +e
            diff --exclude=lib.rs --recursive --unified $src/utils/subxt/generated/src/ ${self'.packages.dali-subxt-client}/ > $out/${name}.patch            
            if [[ $? -ne 1 ]] ; then
              echo "Failed diff"              
            fi              
            set -e 
          '';
          dontFixup = true;
          dontStrip = true;
        };

        centauri-patched-src = pkgs.stdenv.mkDerivation rec {
          name = "centauri-patched-src";
          pname = "${name}";
          buildInputs = [ self'.packages.dali-subxt-client ];
          src = centauri-src;
          patches = [ "${dali-subxt-patch}/dali-subxt-patch.patch" ];
          patchFlags = "--strip=4";
          installPhase = ''
            mkdir --parents $out
            cp --recursive --no-preserve=mode,ownership $src/. $out/
            cd $out/utils/subxt/generated/src
            patch ${patchFlags} -- < ${builtins.head patches}
          '';
          dontFixup = true;
          dontStrip = true;
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

        hyperspace-dali = crane.stable.buildPackage rec {
          name = "hyperspace-dali";
          pname = "${name}";
          cargoArtifacts = crane.stable.buildDepsOnly {
            src = centauri-patched-src;
            doCheck = false;
            cargoExtraArgs = "--package hyperspace --features dali";
            cargoTestCommand = "";
            BuildInputs = [ pkgs.protobuf ];
            PROTOC = "${pkgs.protobuf}/bin/protoc";
            PROTOC_INCLUDE = "${pkgs.protobuf}/include";
            PROTOC_NO_VENDOR = "1";
          };
          src = centauri-patched-src;
          BuildInputs = [ pkgs.protobuf ];
          PROTOC = "${pkgs.protobuf}/bin/protoc";
          PROTOC_INCLUDE = "${pkgs.protobuf}/include";
          PROTOC_NO_VENDOR = "1";
          doCheck = false;
          cargoExtraArgs = "--package hyperspace --features dali";
          cargoTestCommand = "";
          meta = { mainProgram = "hyperspace"; };
        };
      };
    };
}
