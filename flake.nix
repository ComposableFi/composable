{
  description = "Composable Finance";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs }:
    let
      supportedSystems =
        [ "x86_64-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; });
    in {
      nixopsConfigurations.default =
        let pkgs = import nixpkgs {};
        in (pkgs.callPackage ./devnet/default.nix { inherit nixpkgs; }).machines;

      packages = forAllSystems (system:
        let
          pkgs = nixpkgsFor.${system};
          devnet = pkgs.callPackage ./devnet { inherit nixpkgs; };
          latest-book = pkgs.callPackage ./book {};
        in {
          dali-script = devnet.dali.script;
          picasso-script = devnet.picasso.script;
          inherit (devnet.dali) book;
          inherit (devnet) nixops;
          inherit latest-book;
        });

      # Default package is currently the book, but that will change
      defaultPackage = forAllSystems (system: self.packages.${system}.book);

      devShells = forAllSystems (system:
        let pkgs = nixpkgsFor.${system};
        in {
          book = pkgs.mkShell { buildInputs = [ pkgs.mdbook ]; };
          devnet = pkgs.mkShell {
            buildInputs =
              let p = self.packages.${system};
              in [ p.nixops p.dali-script p.picasso-script ];
            NIX_PATH = "nixpkgs=${pkgs.path}";
          };
          main = pkgs.mkShell { # not working yet, needs rust nightly
            buildInputs = [ 
              pkgs.cargo 
              pkgs.rustc
              pkgs.rustfmt
            ];
            nativeBuildInputs = [
                pkgs.llvmPackages.libclang
                pkgs.llvmPackages.libcxxClang
                pkgs.clang
            ];

            # 
            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
            BINDGEN_EXTRA_CLANG_ARGS = "-isystem ${pkgs.llvmPackages.libclang.lib}/lib/clang/${pkgs.lib.getVersion pkgs.clang}/include";
            PROTOC = "${pkgs.protobuf}/bin/protoc";
            ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";

            # Certain Rust tools won't work without this
            # This can also be fixed by using oxalica/rust-overlay and specifying the rust-src extension
            # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela. for more details.
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          };
        });
    };
}
