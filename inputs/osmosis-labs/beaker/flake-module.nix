{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, systemCommonRust
    , subnix, ... }: {
      packages = rec {
        beaker = let
          name = "beaker";
          src = pkgs.fetchFromGitHub {
            owner = "dzmitry-lahoda-forks";
            repo = name;
            rev = "d6700f03da8e93de3e5e327c2e87d93fde3967b8";
            hash = "sha256-42j/ZP8Gyn1gyPTLNMpWJH5pLqz2Ufd8hSb6g8SnkCU=";
          };
          env = {
            doCheck = false;
            buildInputs = with pkgs; [ openssl zstd protobuf ];
            nativeBuildInputs = with pkgs;
              [ clang pkg-config perl ] ++ lib.optional stdenv.isDarwin
              (with pkgs.darwin.apple_sdk.frameworks; [
                Security
                SystemConfiguration
              ]);
            RUST_BACKTRACE = "full";
          } // subnix.subattrs;

        in crane.stable.buildPackage (env // {
          name = name;
          pname = "beaker";
          cargoArtifacts = crane.stable.buildDepsOnly (env // {
            inherit src;
            doCheck = false;
            cargoTestCommand = "";
          });
          inherit src;
          cargoTestCommand = "";
          meta = { mainProgram = name; };
        });
      };
    };
}
