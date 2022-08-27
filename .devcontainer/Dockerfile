FROM composablefi/composable-devcontainer:latest

ARG USER=vscode

# NOTE: keep in sync with base
ARG CHANNEL_URL=https://nixos.org/channels/nixpkgs-22.05-darwin
ARG CACHIX_NAME=composable-community

USER ${USER}


RUN nix-channel --add ${CHANNEL_URL} nixpkgs
RUN nix-channel --update
RUN nix-env -iA nixpkgs.cachix
RUN cachix use ${CACHIX_NAME}