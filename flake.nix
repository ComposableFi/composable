{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    let
      supportedSystems = [ "x86_64-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; });
    in {
      packages = forAllSystems (system:
        let
          pkgs = nixpkgsFor.${system};
        in
        {
          book = pkgs.runCommand "book" {} ''
            cd book
            mdbook build
            mkdir -p $out
            cp -r ./book/* $out
          '';
        }
      );

      devShell = forAllSystems(system: with nixpkgsFor.${system}; mkShell {
        buildInputs = [ mdbook ];
      });
    };
}
