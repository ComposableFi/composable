
# Nix

Assuming you followed official guidance to install Nix with Flakes enabled.

Reference in your flake:

```nix
inputs = {
  composable = {
    url = "github:ComposableFi/composable";
    inputs.nixpkgs.follows = "nixpkgs";
  };
};
```


Shell into dev env:

```bash
nix develop --impure
```

Format all:

```bash
nix run ".#fmt"
```

Check/lint all:

```bash
nix run ".#check"
```
 
Ask GPT or search internet (or just Github) for any issues and debugging.

# Long

Nix is a requirement to set up and start a local development environment with Composable's code. We recommend using the Zero-to-Nix installer. Refer to our docs for how to [install and configure Nix](nix/install.md).

After configuration, familiarize yourself with the following commands:

`nix run "composable#devnet-picasso"` to run local devnet for Polkadot CosmWasm development.

`nix run "composable#fmt"` format all files.

`nix build "composable#unit-tests"` check unit tests.


### Per commit examples


```shell
# run Composable node
nix run "github:ComposableFi/composable/<COMMIT>" --allow-import-from-derivation --extra-experimental-features "flakes nix-command" --no-sandbox --accept-flake-config --option sandbox relaxed
````

```shell
# run local Picasso DevNet (for CosmWasm development)
nix run "github:ComposableFi/composable/<COMMIT>#devnet-picasso" --allow-import-from-derivation --extra-experimental-features "flakes nix-command" --no-sandbox --accept-flake-config --option sandbox relaxed 
```

```shell
# CosmWasm on Substrate CLI tool
nix run "github:ComposableFi/composable/<COMMIT>#ccw" --allow-import-from-derivation --extra-experimental-features "flakes nix-command" --no-sandbox --accept-flake-config --option sandbox relaxed 
```

```shell
# run cross chain devnet with Dotsama and Cosmos nodes 
nix run "github:ComposableFi/composable/<COMMIT>#devnet-xc-fresh" --allow-import-from-derivation --extra-experimental-features "flakes nix-command" --no-sandbox --accept-flake-config --option sandbox relaxed 
# or same with docker
nix build "github:ComposableFi/composable/<COMMIT>#devnet-xc-image" --allow-import-from-derivation --extra-experimental-features "flakes nix-command" --no-sandbox --accept-flake-config --option sandbox relaxed \
&& docker load --input result && docker run -it --entrypoint bash devnet-xc:latest -c /bin/devnet-xc-fresh 
```

# Install Nix

## Installing Nix

Once you have determined which one you want to use, [follow the official Nix
installation instructions](https://zero-to-nix.com/start/install).


## Configuring your Nix install

### On NixOS

In your Nix system config (`/etc/nixos/configuration.nix` by default), configure `nix` like this:

```nix
{
  nix = {
    useSandbox = "relaxed";
    extraOptions = ''
      experimental-features = nix-command flakes
      allow-import-from-derivation = true
    '';
    sandbox = "relaxed";
  };
}
```

### On non-NixOS

Set the contents of `~/.config/nix/nix.conf` to this:

```nix
experimental-features = nix-command flakes
sandbox = relaxed
allow-import-from-derivation = true
```

Append to `/etc/nix/nix.conf`:
```ini
trusted-users = <your user>
```

### Using flags

If you cannot edit these config files, then you can pass the following flags to `nix`. 

```shell
--allow-import-from-derivation --extra-experimental-features "flakes nix-command" --no-sandbox --accept-flake-config --option sandbox relaxed
```

---

You are now ready to start [running packages](./run-packages)!


# Running Nix packages

*Locations and packages*

Before trying to run Nix packages, make sure you have git installed.
You can use `which git` to check for a git installation or run `sudo apt install git` to install it.

Now you can run Nix packages! In order to run one, you need both a **location** and a **package**.

## Locations

Locations are the source of a `git` repository containing a Nix flake (such as [ours](https://github.com/ComposableFi/composable)). For example, a **location** can be:

- `.` for your current directory.
- `github:ComposableFi/composable` for the latest commit on branch main.
- `github:ComposableFi/composable/67b4df903bf8dc2ab0634f9adf9988203a93af27` for commit `67b4df903bf8dc2ab0634f9adf9988203a93af27`. 

Note that for the `github:` locations, you do not need to clone the repository. For `.` you need to clone the repository and `cd` into it.

## Packages

Packages are defined in a repository's `flake.nix`. For example, a **package** can be:

- `composable-book` which builds this book (so meta!).

If you want to see all packages defined by a repository's flake, run `nix flake show "LOCATION"`, for example: `nix flake show "github:ComposableFi/composable"`.

Once you know which **location** and **package** you want, simply run:

```bash
nix run "location#package"
```

```bash
nix run "github:ComposableFi/composable#devnet-picasso"
```

In case of error, append `--print-build-logs --show-trace --debug --keep-derivations --keep-outputs` to command.

## Running in Docker

If you do not have access to `nix`, but you do have access to `docker`, then you can run nix packages within docker as follows:


### Creating a `nix` cache volume
In order to save time on subsequent builds, we create a volume that caches `nix` artifacts:

```bash
docker volume create nix
```

### Running your `location#package`

Make sure you replace `location#package` with your desired **location** and **package**.

```bash
docker run -v /var/run/docker.sock:/var/run/docker.sock -v nix:/nix -p 9988:9988 -it nixos/nix bash -c "nix-env -iA nixpkgs.cachix && cachix use composable && nix run location#package --print-build-logs --extra-experimental-features nix-command --extra-experimental-features flakes" --no-sandbox"
```

---

Now that you are able to run all packages, let's [set up your declarative development environment](./development-environments)!
