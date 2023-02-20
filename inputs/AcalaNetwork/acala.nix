# NOTE: crane can't be used because of how it vendors deps, which is incompatible with some packages in polkadot, an issue must be raised to the repo
{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }: {
      packages = {
        acala-node = let version = "release-karura-2.12.0";
        in pkgs.stdenv.mkDerivation {
          name = "acala";
          version = version;
          src = pkgs.fetchgit {
            url = "https://github.com/AcalaNetwork/Acala.git";
            rev = "refs/heads/${version}";
            sha256 = "sha256-l0vQphfyE0FWISPbu3WvFMifM7mj071kXksntGAXS9k=";
            fetchSubmodules = true;
          };
          installPhase = ''
            mkdir --parents $out/bin && mv ./target/release/acala $out/bin
          '';
          __noChroot = true;
          doCheck = false;
          buildInputs = with pkgs; [ openssl ];
          nativeBuildInputs = with pkgs; [
            clang
            git
            self'.packages.rust-nightly
          ];
          buildPhase = ''
            mkdir home
            export HOME=$PWD/home
            cargo build --locked --features with-all-runtime,rococo-native --profile release --workspace --exclude runtime-integration-tests --exclude e2e-tests --exclude test-service
          '';
          meta = { mainProgram = "acala"; };
          # TODO: moved these to some `cumulus based derivation`
          LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath
            (with pkgs; [ stdenv.cc.cc.lib llvmPackages.libclang.lib ]);
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          PROTOC = "${pkgs.protobuf}/bin/protoc";
          ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
        };
      };
    };
}
