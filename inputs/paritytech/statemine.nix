{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, lib, system, crane
    , systemCommonRust, ... }: {
      packages = {
        statemine-node = let version = "release-parachains-v9360";
        in pkgs.stdenv.mkDerivation (rec {
          name = "statemine-node";
          inherit version;
          pname = "polkadot-parachain";
          src = pkgs.fetchgit {
            url = "https://github.com/paritytech/cumulus.git";
            rev = "refs/heads/${version}";
            sha256 = "sha256-Ue3NkPiZxAKDvEIA5Q06lOS64eu6Mxp9iTDzPr1KYm4=";
            fetchSubmodules = false;
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
            ++ systemCommonRust.darwin-deps;
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
