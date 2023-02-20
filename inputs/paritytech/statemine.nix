{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, lib, system, crane
    , systemCommonRust, ... }: {
      packages = {
        statemine-node = let rev = "release-parachains-v9360";
        in pkgs.stdenv.mkDerivation (rec {
          name = "statemine-node";
          version = rev;
          pname = "polkadot-parachain";
          src = pkgs.fetchFromGitHub {
            repo = "cumulus";
            owner = "paritytech";
            inherit rev;
            hash = "sha256-ExCLnAoheU7auCUnqXN1vfrwTfv2pfF2+bq1Ktii1i0=";
          };
          doCheck = false;
          __noChroot = true;
          buildInputs = with pkgs; [ openssl zstd ];
          configurePhase = ''
            mkdir home
            export HOME=$PWD/home
            export WASM_TARGET_DIRECTORY=$PWD/home
          '';
          buildPhase = ''
            cargo build --release --locked --bin polkadot-parachain --no-default-features
          '';
          installPhase = ''
            mkdir --parents $out/bin && mv ./target/release/polkadot-parachain $out/bin
          '';

          nativeBuildInputs = with pkgs;
            [ self'.packages.rust-nightly clang pkg-config ]
            ++ lib.optional stdenv.isDarwin (with darwin.apple_sdk.frameworks; [
              Security
              SystemConfiguration
            ]);
          LD_LIBRARY_PATH = lib.strings.makeLibraryPath
            (with pkgs; [ stdenv.cc.cc.lib llvmPackages.libclang.lib ]);
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          PROTOC = "${pkgs.protobuf}/bin/protoc";
          ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
          RUST_BACKTRACE = "full";
        });
      };
    };
}
