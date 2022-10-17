FROM mcr.microsoft.com/vscode/devcontainers/base:0.202.7-bullseye
ARG USER=vscode
ARG UID=1000
ARG GID=${UID}
ARG NIX_INSTALLER=https://releases.nixos.org/nix/nix-2.10.3/install
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
    adduser ${USER} root && \
    sed --in-place 's/%sudo.*ALL/%sudo   ALL=(ALL:ALL) NOPASSWD:ALL/' /etc/sudoers && \

RUN mkdir --parents /etc/nix/ && \
    echo "sandbox = relaxed" >> /etc/nix/nix.conf && \
    echo "experimental-features = nix-command flakes" >> /etc/nix/nix.conf && \
    echo "narinfo-cache-negative-ttl = 30" >> /etc/nix/nix.conf  && \
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
    nix-env --set-flag priority 10 nix-2.10.3 && \
    export "ARCH_OS=$(uname -m)-$(uname -s | tr '[:upper:]' '[:lower:]')" && \
    nix build --no-link ".#homeConfigurations.vscode-minimal.${ARCH_OS}.activationPackage" -L --show-trace

# hadolint ignore=SC2086
RUN source ~/.nix-profile/etc/profile.d/nix.sh && \
    export "ARCH_OS=$(uname -m)-$(uname -s | tr '[:upper:]' '[:lower:]')" && \
    "$(nix path-info .#homeConfigurations.vscode-minimal.${ARCH_OS}.activationPackage)"/activate && \
    cachix use ${CACHIX_NAME}
