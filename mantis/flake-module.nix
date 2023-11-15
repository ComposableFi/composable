{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, ... }:
    let
      rust = (self.inputs.crane.mkLib pkgs).overrideToolchain
        (pkgs.rust-bin.stable."1.73.0".default.override {
          targets = [ "wasm32-unknown-unknown" ];
        });
    in {
      packages = rec { };

      devShells.mantis = let
        python-packages = ps: with ps; [ numpy cvxpy wheel virtualenv ];
        python = pkgs.python3.withPackages python-packages;
      in pkgs.mkShell {
        VIRTUALENV_PYTHON="${python}/bin/python3.11";
        VIRTUAL_ENV = 1;
        nativeBuildInputs = [ python ];
        buildInputs = [
          python
          pkgs.virtualenv
          pkgs.conda
          pkgs.pyo3-pack
          rust.cargo
          rust.rustc
        ];
      };
    };
}

