{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }:
    let
      src = (pkgs.callPackage ../../.nix/rust.nix { }).rustSrc;
      cosmwasm-attrs = {
        LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath [
          pkgs.stdenv.cc.cc.lib
          pkgs.llvmPackages.libclang.lib
        ];
        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
        PROTOC = "${pkgs.protobuf}/bin/protoc";
      };

      common-attrs = cosmwasm-attrs // {
        src = src;
        buildInputs = with pkgs; [ openssl zstd ];
        nativeBuildInputs = with pkgs;
          [ clang openssl pkg-config ] ++ pkgs.lib.optional stdenv.isDarwin
          (with darwin.apple_sdk.frameworks; [ Security SystemConfiguration ]);
        doCheck = false;
        cargoCheckCommand = "true";
      };

      buildXcvmContract = name:
        crane.nightly.buildPackage (common-attrs // {
          pnameSuffix = "-${name}";
          cargoArtifacts = null; # https://github.com/ipetkov/crane/pull/186
          cargoLock = common-attrs.src + "/cosmwasm/contracts/Cargo.lock";
          cargoToml = common-attrs.src + "/cosmwasm/contracts/Cargo.toml";
          dummySrc = crane.lib.mkDummySrc {
            src = src;
            cargoLock = common-attrs.src + "/cosmwasm/contracts/Cargo.lock";
          };
          cargoLockContents = builtins.readFile
            (common-attrs.src + "/cosmwasm/contracts/Cargo.lock");

          cargoBuildCommand =
            "cd ./cosmwasm/contracts/ && cargo build --target wasm32-unknown-unknown --profile release --package ${name}";

          doCheck = false;
          cargoCheckCommand = "true";
          RUSTFLAGS = "-C link-arg=-s";
        });
    in {
      packages = {
        xcvm-contract-asset-registry =
          buildXcvmContract "cw-xcvm-asset-registry";
        xcvm-contract-router = buildXcvmContract "cw-xcvm-router";
        xcvm-contract-interpreter = buildXcvmContract "cw-xcvm-interpreter";
      };
    };
}
