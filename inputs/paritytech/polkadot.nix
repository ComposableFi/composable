{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, lib, system, crane, systemCommonRust, ... }:
    let
      substrate = {
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
      buildPolkadotNode =
        { name, version, repo, owner, rev, hash, cargoSha256 }:
        pkgs.rustPlatform.buildRustPackage (rec {
          inherit name version cargoSha256;
          src = pkgs.fetchgit {
            url = "https://github.com/${owner}/${repo}.git";
            inherit rev;
            sha256 = hash;
            fetchSubmodules = false;
          };

          meta = { mainProgram = "polkadot"; };

          __noChroot = true;

        } // substrate);
    in {
      packages = {
        polkadot-node = let version = "v0.9.38";
        in buildPolkadotNode rec {
          name = "polkadot-node";
          inherit version;
          repo = "polkadot";
          owner = "paritytech";
          rev = "refs/tags/${version}";
          hash = "sha256-x2IEIHxH8Hg+jFIpnPrTsqISEAZHFuXhJD+H1S+G3nk=";
          cargoSha256 = "sha256-6/94Uj6MG8VRnV4/yvEwUXdZBCEDSFUgqPTDcK7kiss=";
        };
      };
    };
}
