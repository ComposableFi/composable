# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.224.3/containers/rust/.devcontainer/base.Dockerfile
# This container is inspired by https://levelup.gitconnected.com/vs-code-remote-containers-with-nix-2a6f230d1e4e

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="buster"
FROM mcr.microsoft.com/vscode/devcontainers/base:${VARIANT}

# These dependencies are required by Nix.
RUN apt update -y
RUN apt -y install --no-install-recommends curl xz-utils

# Install Nix 2.4pre20210126.
ARG NIX_INSTALL_SCRIPT=https://github.com/numtide/nix-unstable-installer/releases/download/nix-2.9.0pre20220428_660835d/install
RUN curl -L ${NIX_INSTALL_SCRIPT} | sudo -u vscode NIX_INSTALLER_NO_MODIFY_PROFILE=1 sh

# Configuration for Nix from the repository shared amongst developers.
RUN mkdir -p /etc/nix
COPY nix.conf /etc/nix/nix.conf

# This loads the development environment when the container is started.
COPY profile.sh /etc/profile.d/devcontainer.sh

ENV USER=vscode

# Rebuilding the container should not throw the Nix store away so we make /nix
# a volume. See `devcontainer.json` later.
VOLUME /nix