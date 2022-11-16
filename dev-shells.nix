{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, systemCommonRust, ... }: {
    devShells = rec {
      base = pkgs.mkShell {
        buildInputs = [ inputs'.helix.packages.default ];
        NIX_PATH = "nixpkgs=${pkgs.path}";
      };

      minimal = base.overrideAttrs (base:
        systemCommonRust.common-attrs // {
          buildInputs = base.buildInputs
            ++ (with pkgs; [ clang nodejs python3 yarn ])
            ++ (with self'.packages; [ rust-nightly ]);
          LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath (with pkgs; [
            stdenv.cc.cc.lib
            llvmPackages.libclang.lib
            pkgs.zlib
          ]);
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          PROTOC = "${pkgs.protobuf}/bin/protoc";
          ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
          NIX_PATH = "nixpkgs=${pkgs.path}";
        });

      default = minimal.overrideAttrs (base: {
        buildInputs = base.buildInputs ++ (with pkgs; [
          bacon
          google-cloud-sdk
          jq
          lldb
          llvmPackages_latest.bintools
          llvmPackages_latest.lld
          llvmPackages_latest.llvm
          nix-tree
          nixfmt
          openssl
          openssl.dev
          pkg-config
          qemu
          rnix-lsp
          taplo
          xorriso
          zlib.out
          nix-tree
          nixfmt
          rnix-lsp
          nodePackages.typescript
          nodePackages.typescript-language-server
        ]);
      });

      wasm = default.overrideAttrs (base: {
        buildInputs = base.buildInputs ++ [ pkgs.grub2 ]
          ++ (with self'.packages; [ subwasm wasm-optimizer ]);
      });

      xcvm = wasm.overrideAttrs (base: {
        buildInputs = base.buildInputs ++ (with self'.packages; [ junod gex ]);
        shellHook = ''
          echo "junod alice key:"
          echo "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose" | junod keys add alice --recover --keyring-backend test || true
        '';
      });

      ci = pkgs.mkShell {
        buildInputs = [ pkgs.nixopsUnstable ];
        NIX_PATH = "nixpkgs=${pkgs.path}";
      };
    };
  };
}
