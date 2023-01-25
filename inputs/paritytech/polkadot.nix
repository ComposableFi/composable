{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, lib, system, crane, systemCommonRust, ... }:
    let
      buildPolkadotNode =
        { name, version, repo, owner, rev, hash, cargoSha256 }:
        pkgs.rustPlatform.buildRustPackage rec {
          # HACK: break the nix sandbox so we can build the runtimes. This
          # requires Nix to have `sandbox = relaxed` in its config.
          # We don't really care because polkadot is only used for local devnet.
          inherit name version cargoSha256;

          src = pkgs.fetchFromGitHub { inherit repo owner rev hash; };

          meta = { mainProgram = "polkadot"; };

          # substrate-attrs-node-with-attrs
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
        mmr-polkadot-node = buildPolkadotNode rec {
          name = "mmr-polkadot-v${version}";
          version = "0.9.27";
          repo = "polkadot";
          owner = "ComposableFi";
          rev = "0898082540c42fb241c01fe500715369a33a80de";
          hash = "sha256-dymuSVQXzdZe8iiMm4ykVXPIjIZd2ZcAOK7TLDGOWcU=";
          cargoSha256 = "sha256-u/hFRxt3OTMDwONGoJ5l7whC4atgpgIQx+pthe2CJXo=";
        };

        polkadot-node = buildPolkadotNode rec {
          name = "polkadot-v${version}";
          version = "0.9.33";
          repo = "polkadot";
          owner = "paritytech";
          rev = "v${version}";
          hash = "sha256-3hmoTTzdvC1s0GsfgEz6vaIh/obx+MHCqjnUJR6NRVk=";
          cargoSha256 = "sha256-hmvVfxkZVHN22N5SjUmPZ09XjyO3BtTW1Rh4bim8Ij4=";
        };
      };
    };
}
