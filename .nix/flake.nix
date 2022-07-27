{
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
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils,}: 
    flake-utils.lib.eachDefaultSystem (system: 
          
          let
            overlays = [ (import rust-overlay)]; 
            pkgs = import nixpkgs {
              inherit system overlays;
            };
          in {
            packages = with pkgs; {
              rust = rust-bin."nightly-2022-04-18".default;
            };
          }
    );
}