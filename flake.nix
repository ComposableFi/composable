{
  description = "Composable Finance Local Networks Lancher and documentation Book";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay/2af4f775282ff9cb458c3ef6f30c0a8f689d202b";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      overlays = [ (import rust-overlay) ];
      supportedSystems =
        [ "x86_64-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system overlays; });
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
          main = pkgs.mkShell {
            # source: https://nixos.wiki/wiki/Rust - Installation via rustup
            # tweaks are made to add the wasm32 target
            # TODO: support non-aarch64 architectures

            buildInputs = with pkgs; [
              mdbook
              llvmPackages_latest.llvm
              llvmPackages_latest.bintools
              zlib.out
              xorriso
              grub2
              qemu
              llvmPackages_latest.lld
              python3
              openssl.dev
              pkg-config
              rust-analyzer
              # rustup
              # (rust-bin.stable.latest.default.override {
              #   extensions = [ "rust-src" ];
              # })
              (rust-bin.nightly."2022-02-01".default.override {
                targets = [ "wasm32-unknown-unknown" ];
              })
            ];
            # RUSTC_VERSION = "stable";
            # https://github.com/rust-lang/rust-bindgen#environment-variables
            LIBCLANG_PATH= pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
            HISTFILE=toString ./.history;
            # shellHook = ''
            #   export PATH=$PATH:~/.cargo/bin
            #   export PATH=$PATH:~/.rustup/toolchains/$RUSTC_VERSION-aarch64-unknown-linux-gnu/bin/
            #   rustup target add wasm32-unknown-unknown --toolchain nightly-2022-02-01
            #   '';


            # Disabled because no aarch64 support:
            #
            # Add libvmi precompiled library to rustc search path 
            # RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [ 
            #   pkgs.libvmi
            # ]);


            # Add libvmi, glibc, clang, glib headers to bindgen search path
            BINDGEN_EXTRA_CLANG_ARGS = 
            # Includes with normal include path
            (builtins.map (a: ''-I"${a}/include"'') [
              # Disabled because no aarch64 support:
              # pkgs.libvmi
              pkgs.glibc.dev 
            ])
            # Includes with special directory paths
            ++ [
              ''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
              ''-I"${pkgs.glib.dev}/include/glib-2.0"''
              ''-I${pkgs.glib.out}/lib/glib-2.0/include/''
            ];
            # Old version of BINDGEN_EXTRA_CLANG_ARGS
            # BINDGEN_EXTRA_CLANG_ARGS = "-isystem ${pkgs.llvmPackages.libclang.lib}/lib/clang/${pkgs.lib.getVersion pkgs.clang}/include";
            

            PROTOC = "${pkgs.protobuf}/bin/protoc";
            ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";

            # Disabled because this would need to depend on the nightly version
            # Certain Rust tools won't work without this
            # This can also be fixed by using oxalica/rust-overlay and specifying the rust-src extension
            # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela. for more details.
            # RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          };
        });
    };
}
