{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, lib, system, crane, systemCommonRust, ... }:
    let
      buildPolkadotNode =
        { name, version, repo, owner, rev, hash, cargoSha256 }:
        pkgs.rustPlatform.buildRustPackage rec {
          inherit name version cargoSha256;

          src = pkgs.fetchFromGitHub { inherit repo owner rev hash; };

          meta = { mainProgram = "polkadot"; };

          __noChroot = true;
          doCheck = false;
          buildInputs = with pkgs; [ openssl zstd ];
          nativeBuildInputs = with pkgs;
            [ clang pkg-config ] ++ [ self'.packages.rust-nightly ]
            ++ lib.optional stdenv.isDarwin
            (with pkgs.darwin.apple_sdk.frameworks; [
              Security
              SystemConfiguration
            ]);
          LD_LIBRARY_PATH = lib.strings.makeLibraryPath [
            pkgs.stdenv.cc.cc.lib
            pkgs.llvmPackages.libclang.lib
          ];
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          PROTOC = "${pkgs.protobuf}/bin/protoc";
          ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
        };
    in {
      packages = {
        polkadot-node = let rev = "v0.9.36";
        in buildPolkadotNode rec {
          name = "polkadot-node";
          version = rev;
          repo = "polkadot";
          owner = "paritytech";
          inherit rev;
          hash = "sha256-O0zAoqvLAwiVuR4IpTS9bFHRSo6H1QsKCQofBZsZnWU";
          cargoSha256 = "sha256-sXkOP3rITPHvQX2bzTdySgmKcbGJqzj1vAme21lZQDA=";
        };
      };
    };
}
