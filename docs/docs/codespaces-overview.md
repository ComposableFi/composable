# Codespaces

:::danger
Codespaces is not yet fully compatible with our new build and developer environment system, [Nix](./nix-overview.md).
The recommended approach is to follow our [Nix Guide](./nix-overview) and work locally for now.
:::

At Composable Finance, we use [Codespaces](https://github.com/features/codespaces) in order to provide 
**blazing fast cloud developer environments.** This means that we all use the same 
Dockerized devcontainer environment. It moves all of the resource-intensive processing 
such as code compilation and analysis to the cloud, while the UI is rendered locally, providing a 
**fast, responsive, and consistent user experience**.

This gives us the following advantages:

- All configuration of developer tooling is already done for you and shared with your team. If something works for one 
  person, it works for everyone. This ensures that you can instantly start coding. No more "It works on my machine."
- You leverage fast (`16-core CPU`, `32GB RAM`) machines to compile and run your code. Which speeds up your compilation 
  times, and saves you resources on your main machine.
- You don't have to worry about using the wrong versions of dependencies, build systems, or other tools, since everyone 
  uses the same dependencies that have been defined in code, and are identical to the ones used in our CI pipelines.
- If you come up with an improvement to our development experience, you can open an PR for it and then the entire team 
- can take advantage of it.

Let's [get started with Codespaces!](./codespaces/getting-started.md)
