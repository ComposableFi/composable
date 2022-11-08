# Defining your own packages

*When building something new*

If you are working on something new within Composable that does not have a Nix package yet, you will need to "nixify" your project by writing a package definition in our `flake.nix`. To do this, you should read the following:

- [The Nix language](https://nixos.wiki/wiki/Overview_of_the_Nix_Language). (Don't worry, it is a very simple language)
- [The Nix Flake system](https://nixos.wiki/wiki/Flakes). (Also very simple)
- [Nix Flakes: an Introduction](https://xeiaso.net/blog/nix-flakes-1-2022-02-21). (This one is _very difficult_ to understand. Just kidding: also simple)

Your package should probably live inside of [our monorepo](https://github.com/ComposableFi/composable). If this is the case, you should expand our existing `flake.nix`. However, if this is not the case, and if your package still needs to reference packages that are defined by our monorepo, then you should add our repository as one of your flake's inputs like this:


```nix
inputs = {
  composable = {
    url = "github:ComposableFi/composable";
    inputs.nixpkgs.follows = "nixpkgs";
  };
};
```

Your package is probably similar to one of ours, so you will most likely be able to adapt the definition of one of our packages. However, if this is not the case, you should look at the [trivial builders section in the nixpkgs manual](https://nixos.org/manual/nixpkgs/stable/#chap-trivial-builders).

If you have any further questions, feel free to ask them in the `#nix` channel on Slack.

---

If your packaged service is part of a bigger composition, then here's [how to compose services with Arion](./composing-services-with-arion)!
