{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, ... }: {
    packages = rec {
      hyperspace-dali = let
        src = pkgs.stdenv.mkDerivation {
          name = "centauri-src";
          buildInputs = [ self'.packages.dali-subxt-client ];
          src = pkgs.fetchFromGitHub {
            owner = "ComposableFi";
            repo = "centauri";
            rev = "ecefd88ad0ddcb2b55553c23f985f540bee7a80b";
            hash = "sha256-sHrYRsVbbBwF2XqUVfBDdXP99K8s8A7cwL30+TUvrrs=";
          };
          installPhase = ''
            mkdir $out
            cp -a $src/. $out/
            chmod u+w $out/utils/subxt/generated/src/{parachain.rs,relaychain.rs}
            cp ${self'.packages.dali-subxt-client}/* $out/utils/subxt/generated/src/
          '';
        };

      in crane.stable.buildPackage {
        name = "hyperspace-dali";
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
