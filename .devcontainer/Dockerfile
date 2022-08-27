#FROM composable-devcontainer:z5r445ng6yd5ixa2znpkvxxwl3yvmh84

ARG USER=vscode
ARG UID=1000
ARG GID=${UID}
ARG NIX_INSTALLER=https://releases.nixos.org/nix/nix-2.10.3/install

FROM composablefi/composable-devcontainer:latest

SHELL [ "/bin/bash", "-o", "pipefail", "-o", "errexit", "-c" ]


ARG USER
ARG UID
ARG GID
ARG NIX_INSTALLER

RUN apt-get update && apt-get upgrade -y && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    sudo \
    xz-utils    

RUN usermod --append --groups sudo ${USER} --shell /bin/bash
RUN sed --in-place 's/%sudo.*ALL/%sudo   ALL=(ALL:ALL) NOPASSWD:ALL/' /etc/sudoers
RUN echo "sandbox = false" >> /etc/nix.conf
RUN echo "experimental-features = nix-command flakes" >> /etc/nix.conf
RUN mkdir --parents /nix
# `chown --recursive vscode /nix` does not help
RUN rm --recursive --force vscode /nix

# NOTE: keep in sync with base
# ARG CHANNEL_URL=https://nixos.org/channels/nixpkgs-22.05-darwin
# ARG CACHIX_NAME=composable-community

# VOLUME /nix
USER ${USER}
ENV USER=${USER}
RUN curl --location ${NIX_INSTALLER} > ~/install.sh
RUN chmod +x ~/install.sh
RUN ~/install.sh

# no idea why profile was not created, some rule `if in docker ...` may be
RUN export NIX_PROFILE=$(find /nix -name nix.sh) && \
    chmod +x $NIX_PROFILE && \
    echo "source $NIX_PROFILE" >> ~/.bashrc && \
    echo "source $NIX_PROFILE" >> ~/.profile && \
    echo "source $NIX_PROFILE" >> ~/.bash_profile

#nix-channel --add ${CHANNEL_URL} nixpkgs && \
#   nix-channel --update 
#&& \
# nix-env -iA nixpkgs.cachix && \
# cachix use ${CACHIX_NAME}
