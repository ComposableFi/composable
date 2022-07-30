{
  description = "Composable Finance Local Networks Lancher and documentation Book";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, flake-utils}: 
  flake-utils.lib.eachDefaultSystem(system: 
    nixopsConfigurations.default = null;
  );
}