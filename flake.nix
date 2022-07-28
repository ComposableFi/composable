{

  # done
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, crane, }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };
      rust-toolchain = import ./.nix/rust-toolchain.nix;
    in rec {
      devnet-spec = pkgs.callPackage ./.nix/devnet-spec.nix { inherit nixpkgs; devnet = "devnet"; };
      nixopsConfigurations.default =
        let 
          pkgs = import nixpkgs {};
        in 
          devnet-spec.machines;

      packages = 
        let
          devnet-deploy = pkgs.callPackage ./.nix/devnet-deploy.nix { composable = devnet-spec.composable-repo; polkadot = devnet-spec.polkadot-repo;};
          latest-book = pkgs.callPackage ./book {};
        in {
          dali-script = devnet-deploy.dali.script;
          picasso-script = devnet-deploy.picasso.script;
          inherit (devnet-deploy.dali) composable-book;
          inherit (devnet-deploy) nixops;
          inherit latest-book;
        };

      # TODO: default packages should be our parachain i guess ready to run on the network
      defaultPackage =  self.packages.${system}.composable-book;

      devShells = 
        let 
          PROTOC = "${pkgs.protobuf}/bin/protoc";
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
              qemu
              llvmPackages_latest.lld
              python3
              openssl.dev
              pkg-config
              (rust-bin.${rust-toolchain.toolchain.channel-name}.${rust-toolchain.toolchain.channel-date}.default.override {
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
            

            PROTOC = PROTOC;
            ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";

            # Disabled because this would need to depend on the nightly version
            # Certain Rust tools won't work without this
            # This can also be fixed by using oxalica/rust-overlay and specifying the rust-src extension
            # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela. for more details.
            # RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          };
        };
    });
}
