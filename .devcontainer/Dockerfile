
# ## Maintainance

# Keep in sync nix version and config in these:
# - nix version in our docs to install nix on VM/bare metal (usually locally)
# - ci used https://github.com/cachix/install-nix-action
# - flake.nix

# ## References


# https://github.com/DavHau/nix-portable

# https://www.reddit.com/r/NixOS/comments/uoklud/nix_development_container/

# https://tomferon.com/posts/nix-devcontainer/

# https://github.com/jmgilman/dev-container

# https://github.com/xtruder/nix-devcontainer

# https://github.com/jetpack-io/devbox

# https://github.com/gitpod-io/template-nix

# keep in sync with container default shell user

ARG USER=vscode
ARG UID=1000
ARG GID=${UID}

# NOTE: keep in sync with root flake.nix and CI yamls
ARG NIX_INSTALLER=https://releases.nixos.org/nix/nix-2.10.3/install
ARG CHANNEL_URL=https://nixos.org/channels/nixpkgs-22.05-darwin
ARG CHANNEL_URL=https://nixos.org/channels/nixpkgs-unstable
ARG CACHIX_NAME=composable-community

# 1. inherit from ms based image for nice shell nd sure that runtime injection of vscode will work
# 2. base on image built in CI with nix to prevent long start up time (nix shell on start), and share tolling of all shells
# 3. so until CI with arm, people can built it locally on arm
# 4. docker has no systemd, so not nix out of box daemon (aand nix fails to install if trivial install under root)

#FROM composable-devcontainer:mfqlb4qsqz6wk3lvrz8kfzj137wvlcs5
FROM composablefi/composable-devcontainer:latest

SHELL [ "/bin/bash", "-o", "pipefail", "-o", "errexit", "-c" ]

ARG CHANNEL_URL
ARG USER
ARG UID
ARG GID
ARG NIX_INSTALLER
ARG CACHIX_NAME

# considered nix installer will not work without it, 
# so can try to install it via nix docker tools but if may have its own issues
RUN apt-get update && apt-get upgrade -y && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    sudo \
    xz-utils    

# needed by nix
RUN addgroup --system nixbld
RUN for i in $(seq 1 30); do useradd -ms /bin/bash nixbld$i &&  adduser nixbld$i nixbld; done
RUN adduser ${USER} nixbld

# nix just checks user name != root, so we need some root so to do stuff
RUN usermod --append --groups sudo ${USER} --shell /bin/bash
RUN adduser ${USER} root
RUN sed --in-place 's/%sudo.*ALL/%sudo   ALL=(ALL:ALL) NOPASSWD:ALL/' /etc/sudoers

RUN mkdir --parents /etc/nix/
RUN echo "sandbox = false" >> /etc/nix/nix.conf
RUN echo "experimental-features = nix-command flakes" >> /etc/nix/nix.conf

RUN curl --location ${NIX_INSTALLER} > /install.sh
RUN chmod +x /install.sh
# so you can run docker daemon without knowing """password"""
RUN passwd --delete root


# what ever install for `root`
# not install under root, if try - later user install

# not switch to user
RUN chown --recursive vscode:vscode /nix
# without this line, if our docker has prebuild nix stuff, nix fails to install
RUN chmod -R a+rwx /nix

USER ${USER}
ENV USER=${USER}
RUN /install.sh
RUN source ~/.nix-profile/etc/profile.d/nix.sh && \
    nix-channel --add ${CHANNEL_URL} nixpkgs && \
    nix-channel --update 

RUN source ~/.nix-profile/etc/profile.d/nix.sh && \
    nix-env -iA nixpkgs.cachix
    
RUN source ~/.nix-profile/etc/profile.d/nix.sh && \
    cachix use ${CACHIX_NAME}

RUN echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.bashrc && \
    echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.profile && \
    echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.bash_profile