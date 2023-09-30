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

        yui-ibc-solidity = pkgs.stdenv.mkDerivation {
          name = "yui-ibc-solidity";
          src = pkgs.fetchgit {
            url = "https://github.com/hyperledger-labs/yui-ibc-solidity";
            rev = "d9a90cadaab7c06ddbcf0c7d73ab0c0777cef5a1";
            sha256 = "sha256-X82CvEbvyv/YPT+psnT3cu7n8HxQ8eW0CDIrTC922AA=";
            fetchSubmodules = true;
          };
          phases = [ "installPhase" ];
          installPhase = ''
            cp --recursive --force $src/ $out
          '';
        };

        solidity-bytes-utils = pkgs.stdenv.mkDerivation {
          name = "solidity-bytes-utils";
          src = pkgs.fetchgit {
            url = "https://github.com/GNSPS/solidity-bytes-utils.git";
            rev = "6458fb2780a3092bc756e737f246be1de6d3d362";
            sha256 = "sha256-sJWoYag6hTIoS4Jr1XdqBKfrJaFQ1iMPy+UI5vVb7Lw=";
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
            mkdir --parents $out
            cp --no-preserve=mode,ownership --dereference --recursive --force $src/* $out
            rm --recursive --force $out/lib
            mkdir --parents $out/lib
            function cpf {
              cp --no-preserve=mode,ownership --dereference --recursive --force $@
            }
            cpf "${forge-std}/" $out/lib/forge-std
            cpf "${protobuf3-solidity-lib}/" $out/lib/protobuf3-solidity-lib
            cpf "${openzeppelin-contracts}/" $out/lib/openzeppelin-contracts
            cpf "${yui-ibc-solidity}/" $out/lib/yui-ibc-solidity
            cpf "${solidity-bytes-utils}/" $out/lib/solidity-bytes-utils
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
