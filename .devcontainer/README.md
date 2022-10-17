# Overview

The Nix based devcontainer. Has the same environment as available in CI and on devnets.

If need to configure container, please try `Configure and create codespace -> Dev container configuration -> .devcontainer/*`.

## How to speed up running things in container

### Local

Switch to vscode home manager locally. Mount host /nix/store same path into container

### All

Add our packages into home so that they are prebaked.