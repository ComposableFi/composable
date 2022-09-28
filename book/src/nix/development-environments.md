# Development Environments
_Declarative tooling_

---

We want to make sure that all developers are using identical tools within their shells. We also want to make sure that these tools are identical to the ones being used in our CI pipelines, and that we can declaratively upgrade developer environments. Nix allows us to do this with the [flakes](https://nixos.wiki/wiki/Flakes)' `devShell` system.

For example, 


