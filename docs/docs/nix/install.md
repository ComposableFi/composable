# Install Nix
*On Linux and macOS*.

--- 

## Installing Nix

There are four ways to use Nix at Composable, ranked from most compatible to least compatible:

1. **NixOS**. NixOS is a system that uses Nix to define your entire OS. Switching to it (in a VM) is an excellent way of learning Nix, as you will use it to build and configure your system.
2. **Nix: the package manager on Linux**. Nix on Linux is an (additional) package manager that allows you to build and run Nix packages using the Nix build system. _This is in most cases as compatible as #1_.
3. **Nix in Docker, on macOS**. If you want to run a package on macOS, do not want to deal with a VM, but still want a decent reliability guarantee, then you can run Nix packages within Docker on macOS. _This is incompatible with our declarative development environments and is only meant for running a package_.
4. **Nix on macOS (EXPERIMENTAL)**. You can also use Nix natively on macOS. Nix has excellent cross-system and cross-architecture building support, and a lot of the packages available in the [nixpkgs](https://nixos.wiki/wiki/Nixpkgs) repository support (ARM) macOS. _However, because we use Docker in many of our packages, some packages will not run using this method yet as Docker is exclusive to Linux._

Once you have determined which one you want to use, [follow the official Nix installation instructions](https://nixos.org/download.html).
_If you picked option 3, you can skip to the bottom of the page._

_If you're wondering what the author is using: I'm personally using NixOS within a Parallels VM on my M1 Mac. DM me (**@cor**) on Slack if you also want this._

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
