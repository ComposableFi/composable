# Install Nix

## Installing Nix

There are two supported ways of using Nix at Composable. 

1. **NixOS**. NixOS is a system that uses Nix to define your entire OS.
Switching to it (in a VM) is an excellent way of learning Nix, as you will
use it to build and configure your system. 
2. **Nix: the package manager on Linux**. Nix on Linux is an (additional)
package manager that allows you to build and run Nix packages using the Nix
build system. _This is in most cases as compatible as #1_.

Once you have determined which one you want to use, [follow the official Nix
installation instructions](https://nixos.org/download.html).

### Best-effort, zero-support options

:::danger
**We do not provide any official support for macOS. You are expected to run
Linux if you work at Composable. If you really want to use a Mac, then you are
responsible for managing your own Linux VM. You can look at [my nixos-config](https://github.com/cor/nixos-config), 
where I provide instructions on how to
set up a NixOS VM on an M1 Mac. There is no guarantee that these instructions
will remain up-to-date**
:::

1. **Nix in Docker, on macOS**. If you want to run a package on macOS, do not
want to deal with a VM, but still want a decent reliability guarantee, then
you can run Nix packages within Docker on macOS. _This is incompatible with our
declarative development environments and is only meant for running a package_.

2. **Nix on macOS**. You can also use Nix natively on macOS.
Nix has excellent cross-system and cross-architecture building support, and a
lot of the packages available in the [nixpkgs](https://nixos.wiki/wiki/Nixpkgs)
repository support (ARM) macOS. _However, because we use Docker in many of
our packages, some packages will not run using this method yet as Docker is
exclusive to Linux._

:::caution
Any package that happens to work on macOS needs to be explicitly enabled in
`darwin-filter.nix`. A package working on macOS now is no guarantee that it
will work on macOS in the future. You are still expected to run Linux if you
work at Composable.
:::



## Configuring your Nix install

### On NixOS

In your Nix system config (`/etc/nixos/configuration.nix` by default), configure `nix` like this:

```nix
{
  nix = {
    useSandbox = "relaxed";
    extraOptions = ''
      experimental-features = nix-command flakes
    '';
  };
}
```

### On non-NixOS

Set the contents of `~/.config/nix/nix.conf` to this:

```nix
experimental-features = nix-command flakes
sandbox = relaxed
```

### Using flags

If you are in an environment where you cannot edit these config files, then you can pass the following flags to `nix`. 

```
--extra-experimental-features nix-command --extra-experimental-features flakes --no-sandbox
```

---

You are now ready to start [running packages](./run-packages)!
