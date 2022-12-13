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

- `devnet-dali` which runs a devnet containing `dali` and `rococo`, launched by `polkadot-launch`.
- `frontend-pablo-server` which runs a server serving the `pablo` frontend.
- `composable-book` which builds this book (so meta!).

If you want to see all packages defined by a repository's flake, run `nix flake show "LOCATION"`, for example: `nix flake show "github:ComposableFi/composable"`.


## Running

Once you know which **location** and **package** you want, simply run:

```bash
nix run "location#package" -L
```

For example, if you want to run `frontend-server-pablo` for the current `main` composable branch, run:

```bash
nix run "github:ComposableFi/composable#frontend-server-pablo" -L
```

_Note: the `-L` is optional, but provides you with full logs which is useful for debugging._

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
docker run -v /var/run/docker.sock:/var/run/docker.sock -v nix:/nix -p 9988:9988 -it nixos/nix bash -c "nix-env -iA nixpkgs.cachix && cachix use composable-community && nix run location#package -L --extra-experimental-features nix-command --extra-experimental-features flakes --no-sandbox"
```

---

Now that you are able to run all packages, let's [set up your declarative development environment](./development-environments)!
