# Overview

The Nix based devcontainer. Has the same environment as available in CI and on devnets.

If need to configure container, please try `Configure and create codespace -> Dev container configuration -> .devcontainer/*` .

## Maintainance

Keep in sync nix version and config in these:

- nix version in our docs to install nix on VM/bare metal (usually locally)

- https://github.com/jmgilman/dev-container

- https://github.com/cachix/install-nix-action

## Making container 

Having `nix`ified dev container is hard for several reasons.

First, Microsoft bases images are dynamic and represent sandwich of container build, then runtime build and install (injection) and extensions install. These are not nixos compatible. Generally that process is not reproducible.

Second, fast ARM builders are not yet widespread.

Third, nix likes either root with systemd or non root without systemd, but containers are by default root without systemd. Adding channels is user operation, but updating is root one and it fails in docker.

Forth, we want all tools find all stuff on start
and there is no need to run commands after start. Cooking all before hand is harder.

Five, we have our own requirements to tune in.

Six, not all people are nixos from start (some get used to old style dev), nixos out of box has not so great integration with remote shell. Packaging nix into any container via nix does not really install all needed by nix.

## References


https://github.com/DavHau/nix-portable

https://www.reddit.com/r/NixOS/comments/uoklud/nix_development_container/

https://tomferon.com/posts/nix-devcontainer/

https://github.com/xtruder/nix-devcontainer

https://github.com/jetpack-io/devbox