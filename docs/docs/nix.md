# Nix
*Reproducible builds, developer environments, service orchestrations, CI checks, and more!*
 

At Composable, we use [Nix](https://nixos.org/) to build all of our products and services with a single tool. This ensures that our builds are **reproducible**, **declarative**, and **reliable**.

Nix is **the only build tool** you will need at Composable. There is no need to install `cargo`, `rustup`, `tsc`, `node`, `cmake`, `libssl`, or any other system package. Our dependencies, including system dependencies, and env vars, are declared in our Nix configurations.

Nix also defines our **declarative development environments**. So you do not have to set up any packages on your machine to start developing. Instead, you only need a Nix install, and running `nix run develop` in our repository.

Nix provides us with a **single, uniform interface** across all products. You do not need to understand the underlying build tools for our `frontend-pablo-server` or our `devnet-dali`. You just run both of them with `nix run "#frontend-pablo-server"` and `nix run ".#devnet-dali"`.

Nix also cuts your check-fix feedback-loop by **running all CI checks locally**, and provides you with helpful deterministic utilities like **formatting the entire repository with a single command**.

---

Let's get started and [install Nix](./nix/install)!

