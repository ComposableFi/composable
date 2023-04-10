FROM mcr.microsoft.com/vscode/devcontainers/base:0.202.7-bullseye
ARG USER=vscode
ARG UID=1000
ARG GID=${UID}
ARG NIX_VERSION=2.14.1
ARG NIX_INSTALLER=https://releases.nixos.org/nix/nix-${NIX_VERSION}/install
ARG CHANNEL_URL=https://github.com/NixOS/nixpkgs/archive/aaa1c973c8c189195e1b1a702d3b74dbcde91538.tar.gz
ARG CACHIX_NAME=composable-community

SHELL [ "/bin/bash", "-o", "pipefail", "-o", "errexit", "-c" ]

RUN export DEBIAN_FRONTEND=noninteractive && \
    apt-get update && \
    apt-get install --yes --no-install-recommends \
    ca-certificates \
    curl \
    sudo \
    xz-utils

RUN usermod --append --groups sudo ${USER} --shell /bin/bash && \
    usermod --append --groups root ${USER} --shell /bin/bash && \
    adduser ${USER} root && \
    sed --in-place 's/%sudo.*ALL/%sudo   ALL=(ALL:ALL) NOPASSWD:ALL/' /etc/sudoers

RUN mkdir --parents /etc/nix/ && \
    echo "sandbox = false" >> /etc/nix/nix.conf && \
    echo "experimental-features = nix-command flakes" >> /etc/nix/nix.conf && \
    echo "narinfo-cache-negative-ttl = 30" >> /etc/nix/nix.conf  && \
    echo "trusted-users = root vscode" >> /etc/nix/nix.conf  && \
    echo "substitute = true" >> /etc/nix/nix.conf  && \
    echo "substituters = https://cache.nixos.org/ https://composable-community.cachix.org/ https://devenv.cachix.org/ https://nix-community.cachix.org/" >> /etc/nix/nix.conf  && \
    echo "require-sigs = false" >> /etc/nix/nix.conf  && \
    echo "trusted-public-keys = cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY= composable-community.cachix.org-1:GG4xJNpXJ+J97I8EyJ4qI5tRTAJ4i7h+NK2Z32I8sK8= devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw= nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs= nixpkgs-update.cachix.org-1:6y6Z2JdoL3APdu6/+Iy8eZX2ajf09e4EE9SnxSML1W8=" >> /etc/nix/nix.conf  && \
    echo "trusted-substituters = https://cache.nixos.org/ https://composable-community.cachix.org/ https://devenv.cachix.org/ https://nix-community.cachix.org/" >> /etc/nix/nix.conf  && \
    passwd --delete root

USER ${USER}
ENV USER=${USER}

RUN curl --location ${NIX_INSTALLER} > ~/install.sh && \
    chmod +x ~/install.sh  && \
    ~/install.sh

RUN source ~/.nix-profile/etc/profile.d/nix.sh && \
    nix-channel --add ${CHANNEL_URL} nixpkgs && \
    nix-channel --update 

RUN echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.bashrc && \
    echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.profile && \
    echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.bash_profile && \
    echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.zshrc

WORKDIR /home/${USER}/

COPY --chown=${USER}:${USER} . . 

RUN source ~/.nix-profile/etc/profile.d/nix.sh && \
    nix-env --set-flag priority 10 nix-${NIX_VERSION} && \
    export "ARCH_OS=$(uname -m)-$(uname -s | tr '[:upper:]' '[:lower:]')" && \
    nix build --no-link ".#homeConfigurations.${USER}.activationPackage" --print-build-logs --show-trace


# hadolint ignore=SC2086
RUN source ~/.nix-profile/etc/profile.d/nix.sh && \
    export "ARCH_OS=$(uname -m)-$(uname -s | tr '[:upper:]' '[:lower:]')" && \
    "$(nix path-info .#homeConfigurations.${USER}.activationPackage)"/activate && \
    cachix use ${CACHIX_NAME}
