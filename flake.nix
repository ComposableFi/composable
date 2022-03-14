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
          book = pkgs.callPackage ./book {};
        in {
          inherit (devnet) dali;
          inherit (devnet) picasso;
          inherit (devnet) nixops;
          inherit book;
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
              in [ p.nixops p.dali p.picasso ];
            NIX_PATH = "nixpkgs=${pkgs.path}";
          };
        });
    };
}
