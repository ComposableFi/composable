{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }:
    let nix = " --offline --out $out/lib ";
    in {

      packages = rec {
        openzeppelin-contracts = pkgs.stdenv.mkDerivation {
          name = "openzeppelin-contracts";
          src = pkgs.fetchgit {
            url = "https://github.com/OpenZeppelin/openzeppelin-contracts.git";
            rev = "fd81a96f01cc42ef1c9a5399364968d0e07e9e90";
            sha256 = "sha256-ggdq/9VgvxeFez8ouJpjRtBwOtJFgmVgOqTkzCb83oc=";
            fetchSubmodules = true;
          };
          phases = [ "installPhase" ];
          installPhase = ''
            cp --recursive --force $src/ $out
          '';
        };
        forge-std = pkgs.stdenv.mkDerivation {
          name = "forge-std";
          src = pkgs.fetchgit {
            url = "https://github.com/foundry-rs/forge-std.git";
            rev = "1d9650e951204a0ddce9ff89c32f1997984cef4d";
            sha256 = "sha256-KJU6k0w8ZCE8WXgwySBpatPzgf6jydB3cbKAiJIwWQY=";
            fetchSubmodules = true;
          };
          phases = [ "installPhase" ];
          installPhase = ''
            cp --recursive --force $src/ $out
          '';
        };

        protobuf3-solidity-lib = pkgs.stdenv.mkDerivation {
          name = "protobuf3-solidity-lib";
          src = pkgs.fetchgit {
            url = "https://github.com/lazyledger/protobuf3-solidity-lib.git";
            rev = "bc4e75a0bf6e365e820929eb293ef9b6d6d69678";
            sha256 = "sha256-+HHUYhWDNRgA7x7p3Z0l0lS1e6pkJh4ZOSCCS4jQZQk=";
            fetchSubmodules = true;
          };
          phases = [ "installPhase" ];
          installPhase = ''
            cp --recursive --force $src/ $out
          '';
        };

        evm-cvm-src = pkgs.stdenv.mkDerivation {
          name = "evm-cvm-src";
          src = ./.;
          phases = [ "installPhase" ];
          installPhase = ''
            mkdir --parents $out/lib
            cp --no-preserve=mode,ownership --dereference --recursive --force $src/* $out
            cp --no-preserve=mode,ownership --dereference --recursive --force  "${openzeppelin-contracts}/" $out/lib/openzeppelin-contracts
            cp --no-preserve=mode,ownership --dereference --recursive --force "${forge-std}/" $out/lib/forge-std
            cp --no-preserve=mode,ownership --dereference --recursive --force "${protobuf3-solidity-lib}/" $out/lib/protobuf3-solidity-lib
          '';
        };

        evm-cvm-gateway = pkgs.stdenv.mkDerivation rec {
          name = "evm-cvm-gateway";
          FOUNDRY_SOLC = "${pkgs.solc}/bin/solc";
          nativeBuildInputs = [ self'.packages.forge pkgs.solc ];
          src = evm-cvm-src;
          patchPhase = "true";
          buildPhase = "true";
          installPhase = ''
            mkdir --parents $out/lib
            forge build ${nix}
          '';
          dontFixup = true;
          dontStrip = true;
        };
      };
    };
}
