{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }:
    let
      bifrost-src = pkgs.fetchFromGitHub {
        owner = "bifrost-finance";
        repo = "bifrost";
        rev = "5f4bf0c8decd8e59cca007fda1dc3364f7cd4245";
        hash = "sha256-E7mLS9kQOwQJxvYd8cfEu5yd1J0U+n/2ZKXui4DayPA=";
      };
      prelude = {
        inherit (pkgs) lib;
        inherit (self'.packages) rust-nightly;
      };
    in with prelude; {
      packages = rec {
        bifrost-node = pkgs.stdenv.mkDerivation (rec {
          name = "bifrost";
          pname = "bifrost";
          src = bifrost-src;
          doCheck = false;
          __noChroot = true;
          buildInputs = with pkgs; [ openssl zstd ];
          configurePhase = ''
            	mkdir home
              export HOME=$PWD/home
              export WASM_TARGET_DIRECTORY=$PWD/home
          '';
          buildPhase = ''
            cargo build --release --locked --bin ${name} --no-default-features --features="cli"
          '';
          installPhase = ''
            mkdir --parents $out/bin && mv ./target/release/${name} $out/bin
          '';
          nativeBuildInputs = with pkgs;
            [ rust-nightly clang pkg-config ] ++ lib.optional stdenv.isDarwin
            (with darwin.apple_sdk.frameworks; [
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
