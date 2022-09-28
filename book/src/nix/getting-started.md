# Getting Started
*On Linux and macOS*.

--- 

## Installing Nix

There are four ways in order to use Nix at Composable, ranked from most compatible to least compatible:

1. **NixOS**. NixOS is a system that uses Nix in order to define your entire OS. Switching to it (in a VM) is a great way of learning Nix, as you will use it to build and configure your own system.
2. **Nix: the package manager, on Linux**. Nix on Linux is an (additional) package manager that allows you to build and run Nix packages using the Nix build system. _This is in most cases as compatible as #1_.
3. **Nix in Docker, on macOS**. If you want to run a package on macOS, do not want to deal with a VM, but still want a decent reliablity guarantee, then you can run Nix packages within Docker on macOS. _This is not compatible with our declarative development environments and is only meant for running a package_.
4. **Nix on macOS (EXPERIMENTAL)**. You can also use Nix natively on macOS. Nix has excellent cross-system and cross-architecture building support, and a lot of the packages available in the [nixpkgs](https://nixos.wiki/wiki/Nixpkgs) repository support (ARM) macOS. _However, because we use Docker in a lot of our packages, some packages will not run using this method yet as Docker is exclusive to Linux._

Once you have determined which one you want to use, [follow the official Nix installation instructions](https://nixos.org/download.html).
_If you picked option 3, you can skip to the bottom of the page._

_If you're wondering what the author is using: I'm personally using NixOS within a Parallels VM on my M1 Mac. DM me (**@cor**) on Slack if you also want this._

## Configuring your Nix install

### On NixOS

At your Nix system config (`/etc/nixos/configuration.nix` by default), configure `nix` like this:

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

## Running Nix packages

Now you can run Nix packages! In order to run one, you need both a **location** and a **package or app**.

### Locations

Locations are the source of a `git` repository. For example, a **location** can be:

- `.` for your current directory.
- `github:ComposableFi/composable` for the latest commit on branch main.
- `github:ComposableFi/composable/67b4df903bf8dc2ab0634f9adf9988203a93af27` for commit `67b4df903bf8dc2ab0634f9adf9988203a93af27`. 

Note that for the `github:` locations, you do not need to clone the repository. For `.` you need to clone the repository and `cd` into it.

### Packages

Packages are defined in a repository's `flake.nix`. For example, a **package** can be:

- `devnet-dali` which runs a devnet containing `dali` and `rococo`, launched by `polkadot-launch`.
- `frontend-pablo-server` which runs a server serving the `pablo` frontend.
- `composable-book` which builds this book (so meta!).

If you want to see all packages that are defined by a repository's flake, you run `nix flake show "LOCATION"`, for example: `nix flake show "github:ComposableFi/composable"`.


### Running

Once you know which **location** and **package** you want, simply run:

```bash
nix run "location#package" -L
```

For example, if you want to run `frontend-server-pablo` for the current `main` composable branch, run:

```bash
nix run "github:ComposableFi/composable#frontend-server-pablo" -L
```

_Note: the `-L` is optional, but provides you with full logs which is useful for debugging._

### Running in Docker

If you do not have access to `nix`, but you do have access to `docker`, then you can run nix packages within docker like this:


#### Creating a `nix` cache volume
In order to save time on subsequent builds, we create a volume that caches `nix` artifacts:

```bash
docker volume create nix
```

#### Running your `location#package`

Make sure you replace `location#package` with your desired **location** and **package**.

```bash
docker run -v /var/run/docker.sock:/var/run/docker.sock -v nix:/nix -p 9988:9988 -it nixos/nix bash -c "nix-env -iA nixpkgs.cachix && cachix use composable-community && nix run location#package -L --extra-experimental-features nix-command --extra-experimental-features flakes --no-sandbox"
```

