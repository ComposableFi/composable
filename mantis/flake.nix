{
  description = "Mantis for MEV";

  inputs = { nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable"; };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems =
        [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
      perSystem = { config, self', inputs', pkgs, system, ... }: {
        packages.default = pkgs.hello;
        formatter = pkgs.alejandra;
        devShells.default = let
          python-packages = ps: with ps; [ numpy cvxpy wheel virtualenv ];
          python = pkgs.python3.withPackages python-packages;
        in pkgs.mkShell {
          # VIRTUALENV_PYTHON="${python}/bin/python3.11";
          VIRTUAL_ENV = 1;
          nativeBuildInputs = [ python ];
          buildInputs = with pkgs; [
            python
            virtualenv
            conda
            rustc
            pyo3-pack
            cargo
          ];
        };
      };
    };
}
