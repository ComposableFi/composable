{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }:
    let
      version = "bifrost-v0.9.69";
      bifrost-src = pkgs.fetchFromGitHub {
        owner = "bifrost-finance";
        repo = "bifrost";
        rev = "refs/tags/${version}";
        hash = "sha256-3et1mMGVnBlHxZofPo7a1l8BSz1C4D3P86eRoIDGzxY=";
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
          inherit version;
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
