{
  description = "Composable Finance";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; });
    in {
      packages = forAllSystems (system:
        let pkgs = nixpkgsFor.${system}; in
        {
          book = pkgs.stdenv.mkDerivation {
            name = "composable-book";
            src = ./book;
            buildInputs = [ pkgs.mdbook ];
            dontUnpack = true;
            installPhase = ''
              mkdir -p $out/book
              cd $src
              mdbook build --dest-dir $out/book
              '';
          };
        }
      );

      # Default package is currently the book, but that will change
      defaultPackage = forAllSystems (system: self.packages.${system}.book);

      devShells = forAllSystems(system: 
        let pkgs = nixpkgsFor.${system}; in 
        {
          book = pkgs.mkShell {
            buildInputs = [ pkgs.mdbook ];
          };
        }
      );
    };
}
