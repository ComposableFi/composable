
<br />
<br />

<p align="center">
  <img alt="Composable Finance" title="Composable Finance" src="banner.png">
</p>

<br />
<br />

## Monorepo for Composable Finance

[![Latest Release](https://img.shields.io/github/v/tag/composablefi/composable)][latest-url]
![Build][build-badge]
[![Discord][discord-badge]][discord-url]
[![Mergify Status][mergify-status]][mergify]




[latest-url]: https://github.com/composablefi/composable/tags

[build-badge]: https://github.com/composablefi/composable/actions/workflows/check.yml/badge.svg

[discord-badge]: https://img.shields.io/badge/Discord-gray?logo=discord
[discord-url]: https://discord.gg/pFZn2GCn65

[mergify]: https://dashboard.mergify.com/github/ComposableFi/repo/composable/queues
[mergify-status]: https://img.shields.io/endpoint.svg?url=https://api.mergify.com/v1/badges/ComposableFi/composable&style=flat

## Documentation

To learn more about our ecosystem, vision, and product specifics - visit our 
[mdbook](https://docs.composable.finance).


## Nix

We use [`nix`](https://nixos.org/) in order to reproducibly build our products. We recommend either installing `nix` or switching to `NixOS`. Alternatively, you can run our packages with just `docker` installed.
Our packages support both **x86** and **ARM** architectures.


### Configuration

Once you have `nix` or `NixOS` installed, you should enable the following features:

#### On NixOS
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

#### On non-NixOS
Set the contents of `~/.config/nix/nix.conf` to 

```conf
experimental-features = nix-command flakes
sandbox = relaxed
```

### Building and running packages

You can now use `nix flake show` in order to view all of the packages we provide, such as `composable-node` and `devnet-dali`.

If you want to run the latest version of  `devnet-dali`, for example, you can simply run the following:
(_You do not need to clone the repository in order to run this_)

```bash
nix run "github:ComposableFi/composable#devnet-dali"
```

If you would like to run an older/pinned version of any package, you can include the commit hash in the package identifier lilke this:

```bash
nix run "github:ComposableFi/composable/d735de9#devnet-dali"
```

If you want to build/run packages based on a local copy of the sources, you can do that like this:


```bash
git clone git@github.com:ComposableFi/composable
cd composable
nix run ".#devnet-dali"
```

### Nix within Docker

Do you not feel like installing `nix`? You can also use `nix` within `docker` like this:

```bash
docker volume create nix # cache builds

docker run -v nix:/nix -p 9988:9988 -it nixos/nix bash -c "nix run github:ComposableFi/composable#devnet-dali --extra-experimental-features nix-command --extra-experimental-features flakes"
```


