# Nix
*Reproducible builds, deployments, and developer environments.*
 
---

At Composable, we use [Nix](https://nixos.org/) in order to build all of our products and services with a single tool. This ensures that our builds are **reproducible**, **declarative**, and **reliable**.

Nix is **the only build tool** you will need at Composable. There is no need to install `cargo`, `rustup`, `tsc`, `node`, `cmake`, `libssl`, or any other system package. All of our dependencies, including system dependencies and env vars, are declared in our Nix configurations.

Nix also defines our **declarative development environments**. Meaning that you do not have to set up any packages on your machine in order to start developing. All you need is a Nix install, and running `nix run develop` in our repository.

Nix provides us with a **single, uniform interface** accross all products. You do not need to understand the underlying build tools for our `frontend-pablo-server` or our `devnet-dali`. You just run both of them with `nix run "#frontend-pablo-server"` and `nix run ".#devnet-dali"`.

Let's get started and [install Nix](./nix/install.html)!

