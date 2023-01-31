{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, ... }: {
    packages = let
      hyperspace-src = chainName: subxtClient:
        pkgs.stdenv.mkDerivation rec {
          name = "centauri-${chainName}";
          pname = "${name}";
          buildInputs = [ subxtClient ];
          src = pkgs.fetchFromGitHub {
            owner = "obsessed-cake";
            repo = "centauri";
            rev = "fa7d5d33125fba9aa48c5e581ec72a543abef25b";
            hash = "sha256-3S0HsFLxWHGXGW8QQD0qD3CWMMZ9vvYYZRdMJ9bYSSE=";
          };
          patchPhase = "";
          installPhase = ''
            mkdir $out
            cp --archive $src/. $out/
            chmod u+w $out/utils/subxt/generated/src/{parachain.rs,relaychain.rs}
            cp ${subxtClient}/* $out/utils/subxt/generated/src/
          '';
          dontFixup = true;
          dontStrip = true;
        };

      hyperspace = { chainName, subxtClient }:
        let
          crateFeatures = if chainName == "dali" then "--features dali" else "";
        in crane.stable.buildPackage rec {
          name = "hyperspace-${chainName}";
          pname = "${name}";
          cargoArtifacts = crane.stable.buildDepsOnly {
            src = hyperspace-src chainName subxtClient;
            doCheck = false;
            cargoExtraArgs = "-p hyperspace ${crateFeatures}";
            cargoTestCommand = "";
            BuildInputs = [ pkgs.protobuf ];
            PROTOC = "${pkgs.protobuf}/bin/protoc";
            PROTOC_INCLUDE = "${pkgs.protobuf}/include";
            PROTOC_NO_VENDOR = "1";
          };
          src = hyperspace-src chainName subxtClient;
          BuildInputs = [ pkgs.protobuf ];
          PROTOC = "${pkgs.protobuf}/bin/protoc";
          PROTOC_INCLUDE = "${pkgs.protobuf}/include";
          PROTOC_NO_VENDOR = "1";
          doCheck = false;
          cargoExtraArgs = "-p hyperspace ${crateFeatures}";
          cargoTestCommand = "";
          meta = { mainProgram = "hyperspace"; };
        };
    in rec {
      hyperspace-composable = hyperspace {
        chainName = "composable";
        subxtClient = self'.packages.composable-subxt-client;
      };

      hyperspace-picasso = hyperspace {
        chainName = "picasso";
        subxtClient = self'.packages.picasso-subxt-client;
      };

      hyperspace-dali = hyperspace {
        chainName = "dali";
        subxtClient = self'.packages.dali-subxt-client;
      };

      hyperspace-composable-container = pkgs.dockerTools.buildImage {
        tag = "latest";
        name = "hyperspace-composable";
        config = {
          Entrypoint = [ "${hyperspace-composable}/bin/hyperspace" ];
        };
      };

      hyperspace-picasso-container = pkgs.dockerTools.buildImage {
        tag = "latest";
        name = "hyperspace-dali";
        config = { Entrypoint = [ "${hyperspace-picasso}/bin/hyperspace" ]; };
      };

      hyperspace-dali-container = pkgs.dockerTools.buildImage {
        tag = "latest";
        name = "hyperspace-dali";
        config = { Entrypoint = [ "${hyperspace-dali}/bin/hyperspace" ]; };
      };
    };
  };
}
