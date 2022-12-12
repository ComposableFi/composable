{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }:
    let
      cosmwasm-attrs = {
        LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath [
          pkgs.stdenv.cc.cc.lib
          pkgs.llvmPackages.libclang.lib
        ];
        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
        PROTOC = "${pkgs.protobuf}/bin/protoc";
      };

      common-attrs = cosmwasm-attrs // {
        src = ./.;
        buildInputs = with pkgs; [ openssl zstd ];
        nativeBuildInputs = with pkgs;
          [ clang openssl pkg-config ] ++ pkgs.lib.optional stdenv.isDarwin
          (with darwin.apple_sdk.frameworks; [ Security SystemConfiguration ]);
        doCheck = false;
        cargoCheckCommand = "true";
      };

      common-deps = crane.nightly.buildDepsOnly (common-attrs // { });

      buildXcvmContract = name:
        crane.nightly.buildPackage (common-attrs // {
          pnameSuffix = "-${name}";
          cargoArtifacts = common-deps;
          cargoExtraArgs =
            "--target wasm32-unknown-unknown --profile release-p ${name}";
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
