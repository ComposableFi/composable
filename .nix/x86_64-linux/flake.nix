{
  description = "Pure Nix flake utility functions";
  outputs = { self }: {
    lib = {
      eachDefaultSystem = iterator : 
         [iterator("42")];
    };
  };
}