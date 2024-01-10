FROM mcr.microsoft.com/vscode/devcontainers/base:0.202.7-bullseye

ARG NIX_VERSION=2.18.1
ARG CHANNEL_URL=https://github.com/NixOS/nixpkgs/archive/1db42b7fe3878f3f5f7a4f2dc210772fd080e205.tar.gz

ARG USER=vscode
ARG UID=1000
ARG GID=${UID}
ARG NIX_INSTALLER=https://releases.nixos.org/nix/nix-${NIX_VERSION}/install
ARG CACHIX_NAME=composable

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
    echo "sandbox = relaxed" >> /etc/nix/nix.conf && \
    echo "experimental-features = nix-command flakes" >> /etc/nix/nix.conf && \    
    echo "cores = 32" >> /etc/nix/nix.conf && \
    echo "allow-import-from-derivation = true" >> /etc/nix/nix.conf && \
    echo "narinfo-cache-negative-ttl = 30" >> /etc/nix/nix.conf  && \
    echo "trusted-users = root vscode actions-runner" >> /etc/nix/nix.conf  && \
    echo "substitute = true" >> /etc/nix/nix.conf  && \
    echo "substituters = https://nix-community.cachix.org/ https://cache.nixos.org/ https://composable.cachix.org/ https://devenv.cachix.org/ https://cosmos.cachix.org https://nixpkgs-update.cachix.org" >> /etc/nix/nix.conf  && \
    echo "require-sigs = false" >> /etc/nix/nix.conf  && \
    echo "trusted-public-keys = nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs= cosmos.cachix.org-1:T5U9yg6u2kM48qAOXHO/ayhO8IWFnv0LOhNcq0yKuR8= cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY= composable.cachix.org-1:J2TVJKH4U8xqYdN/0SpauoAxLuDYeheJtv22Vn3Hav8= nixpkgs-update.cachix.org-1:6y6Z2JdoL3APdu6/+Iy8eZX2ajf09e4EE9SnxSML1W8= devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=" >> /etc/nix/nix.conf  && \
    echo "trusted-substituters = https://nix-community.cachix.org/ https://cache.nixos.org/ https://composable.cachix.org/ https://devenv.cachix.org/ https://cosmos.cachix.org https://nixpkgs-update.cachix.org/" >> /etc/nix/nix.conf  && \
    passwd --delete root

USER ${USER}
ENV USER=${USER}

RUN curl --location ${NIX_INSTALLER} > ~/install.sh && \
    chmod +x ~/install.sh  && \
    ~/install.sh

RUN source ~/.nix-profile/etc/profile.d/nix.sh && \
    nix-channel --add ${CHANNEL_URL} nixpkgs && \
    nix-channel --update 

RUN echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.profile && \
    echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.bashrc

WORKDIR /home/${USER}/

COPY --chown=${USER}:${USER} . . 

RUN source ~/.nix-profile/etc/profile.d/nix.sh && \
    nix-env --set-flag priority 10 nix-${NIX_VERSION} && \
    export "ARCH_OS=$(uname -m)-$(uname -s | tr '[:upper:]' '[:lower:]')" && \
    nix build --no-link ".#homeConfigurations.${USER}.activationPackage" --print-build-logs --show-trace --accept-flake-config

RUN source ~/.nix-profile/etc/profile.d/nix.sh && \
    export "ARCH_OS=$(uname -m)-$(uname -s | tr '[:upper:]' '[:lower:]')" && \
    "$(nix path-info .#homeConfigurations.${USER}.activationPackage)"/activate && \
    cachix use ${CACHIX_NAME}

# variables are put into this file, but also some extra vars
# seems shole file conflicts with vscode startup injection
# so getting one by one for evaluation
RUN cat ~/.nix-profile/etc/profile.d/hm-session-vars.sh | grep "CARGO_NET_GIT_FETCH_WITH_CLI" >> ~/.profile
RUN cat ~/.nix-profile/etc/profile.d/hm-session-vars.sh | grep "CARGO_NET_RETRY" >> ~/.profile
RUN cat ~/.nix-profile/etc/profile.d/hm-session-vars.sh | grep "LIBCLANG_PATH" >> ~/.profile
RUN cat ~/.nix-profile/etc/profile.d/hm-session-vars.sh | grep "PROTOC" >> ~/.profile
RUN cat ~/.nix-profile/etc/profile.d/hm-session-vars.sh | grep "ROCKSDB_LIB_DIR" >> ~/.profile

RUN cat ~/.nix-profile/etc/profile.d/hm-session-vars.sh | grep "CARGO_NET_GIT_FETCH_WITH_CLI" >> ~/.bashrc
RUN cat ~/.nix-profile/etc/profile.d/hm-session-vars.sh | grep "CARGO_NET_RETRY" >> ~/.bashrc
RUN cat ~/.nix-profile/etc/profile.d/hm-session-vars.sh | grep "LIBCLANG_PATH" >> ~/.bashrc
RUN cat ~/.nix-profile/etc/profile.d/hm-session-vars.sh | grep "PROTOC" >> ~/.bashrc
RUN cat ~/.nix-profile/etc/profile.d/hm-session-vars.sh | grep "ROCKSDB_LIB_DIR" >> ~/.bashrc