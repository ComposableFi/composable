{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, ... }: {
    packages = rec {
      hyperspace-dali-container = pkgs.dockerTools.buildImage {
        tag = "latest";
        name = "hyperspace-dali";
        config = { Entrypoint = [ "${hyperspace-dali}/bin/hyperspace" ]; };
      };

      hyperspace-dali = let
        src = pkgs.stdenv.mkDerivation rec {
          name = "centauri-src";
          pname = "${name}";
          buildInputs = [ self'.packages.dali-subxt-client ];
          src = pkgs.fetchFromGitHub {
            owner = "obsessed-cake";
            repo = "centauri";
            rev = "fa7d5d33125fba9aa48c5e581ec72a543abef25b";
            hash = "sha256-3S0HsFLxWHGXGW8QQD0qD3CWMMZ9vvYYZRdMJ9bYSSE=";
          };
          installPhase = ''
            mkdir $out
            cp -a $src/. $out/
            chmod u+w $out/utils/subxt/generated/src/{parachain.rs,relaychain.rs}
            cp ${self'.packages.dali-subxt-client}/* $out/utils/subxt/generated/src/
          '';
        };
      in crane.stable.buildPackage rec {
        name = "hyperspace-dali";
        pname = "hyperspace";
        inherit src;
        BuildInputs = [ pkgs.protobuf ];
        PROTOC = "${pkgs.protobuf}/bin/protoc";
        PROTOC_INCLUDE = "${pkgs.protobuf}/include";
        PROTOC_NO_VENDOR = "1";
        doCheck = false;
        cargoExtraArgs = "--features dali";
        cargoTestCommand = "";
        meta = { mainProgram = "hyperspace"; };
      };
    };
  };
}
