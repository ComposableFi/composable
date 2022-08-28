# keep in sync with container default shell user
ARG USER=vscode
ARG UID=1000
ARG GID=${UID}

# NOTE: keep in sync with root flake.nix and CI yamls
ARG NIX_INSTALLER=https://releases.nixos.org/nix/nix-2.10.3/install
ARG CHANNEL_URL=https://nixos.org/channels/nixpkgs-22.05-darwin
ARG CHANNEL_URL=https://nixos.org/channels/nixpkgs-unstable
ARG CACHIX_NAME=composable-community

FROM composablefi/composable-devcontainer:latest
#FROM composable-devcontainer:mfqlb4qsqz6wk3lvrz8kfzj137wvlcs5

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


# what ever install for `root`
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