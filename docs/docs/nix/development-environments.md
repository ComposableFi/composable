# Development Environments
*Declarative tooling*

---

We want to ensure that all developers use identical tools within their shells. We also want to ensure that these tools are identical to those used in our CI pipelines and that we can declaratively upgrade developer environments. Nix allows us to do this with the [flakes](https://nixos.wiki/wiki/Flakes)' `devShell` system.

In order to use our declarative development environment, go to your checked out repository and simply run:

```bash
nix develop
```

This will download all required tools you need and put you into our declarative development environment. From there you can type `hx` in order to load [Helix](https://helix-editor.com/), a very fast editor written in Rust that is preconfigured to use the correct language server. You can also launch any other editor from here that you normally use, such as `code`, `vim`, or `emacs`.

## Alternative `devShell`s

We provide not only the `default` devShell but also other ones optimized for specific purposes. So, for example, you can use:

```bash
nix develop ".#docs"
```

To launch a shell that only includes the required tools for writing docs.

You can view all available devShells by typing `nix flake show`.

---

Once you're done developing a new feature or fixing a bug, you should ensure that CI will pass. So let's use Nix to [reproducibly run all checks locally](./running-checks)!
