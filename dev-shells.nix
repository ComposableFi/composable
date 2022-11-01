{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, systemCommonRust, ... }: {
    devShells = rec {
      base-shell = pkgs.mkShell {
        buildInputs = [ inputs'.helix.packages.default ];
        NIX_PATH = "nixpkgs=${pkgs.path}";
      };

      developers-minimal = base-shell.overrideAttrs (base:
        systemCommonRust.common-attrs // {
          buildInputs = base.buildInputs ++ 
          (with pkgs; [ clang nodejs python3 yarn ]) ++
          (with self'.packages; [ rust-nightly subwasm ]);
          LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath (with pkgs; [
            stdenv.cc.cc.lib
            llvmPackages.libclang.lib
          ]);
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          PROTOC = "${pkgs.protobuf}/bin/protoc";
          ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
          NIX_PATH = "nixpkgs=${pkgs.path}";
        });
    };
  };
}
