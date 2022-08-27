{
  description = "A very basic flake";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };
  outputs = { self, nixpkgs }: rec {
    pkgs = import nixpkgs { system = "x86_64-linux"; };

    packages.x86_64-linux.hello = nixpkgs.legacyPackages.x86_64-linux.hello;

    defaultPackage.x86_64-linux = self.packages.x86_64-linux.hello;
    devShells.x86_64-linux.default = pkgs.mkShell
      ({
        buildInputs = [
          nixpkgs.legacyPackages.x86_64-linux.hello
        ];
      });
  };
}
