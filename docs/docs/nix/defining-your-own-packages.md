# Defining your own packages

Your package can reference packages that are defined here, you should add this repository as one of your flake's inputs like this:

```nix
inputs = {
  composable = {
    url = "github:ComposableFi/composable";
    inputs.nixpkgs.follows = "nixpkgs";
  };
};
```