#FROM composable-devcontainer:z5r445ng6yd5ixa2znpkvxxwl3yvmh84

# keep in sync with container default shell user
ARG USER=vscode
ARG UID=1000
ARG GID=${UID}

# NOTE: keep in sync with base
ARG NIX_INSTALLER=https://releases.nixos.org/nix/nix-2.10.3/install
ARG CHANNEL_URL=https://nixos.org/channels/nixpkgs-22.05-darwin

ARG CACHIX_NAME=composable-community

#FROM composablefi/composable-devcontainer:latest
FROM composable-devcontainer:mfqlb4qsqz6wk3lvrz8kfzj137wvlcs5

SHELL [ "/bin/bash", "-o", "pipefail", "-o", "errexit", "-c" ]

ARG CHANNEL_URL
ARG USER
ARG UID
ARG GID
ARG NIX_INSTALLER
ARG CACHIX_NAME

RUN apt-get update && apt-get upgrade -y && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    sudo \
    xz-utils    

RUN usermod --append --groups sudo ${USER} --shell /bin/bash
RUN usermod --append --groups root ${USER} --shell /bin/bash
RUN sed --in-place 's/%sudo.*ALL/%sudo   ALL=(ALL:ALL) NOPASSWD:ALL/' /etc/sudoers
RUN mkdir --parents /etc/nix/
RUN echo "sandbox = false" >> /etc/nix/nix.conf
RUN echo "experimental-features = nix-command flakes" >> /etc/nix/nix.conf
# && mkdir -p /etc/nix && echo 'sandbox = false' > /etc/nix/nix.conf
RUN mkdir --parents /nix
RUN curl --location ${NIX_INSTALLER} > /install.sh
RUN chmod +x /install.sh
RUN chown --recursive vscode:vscode /nix
#RUN chown --recursive vscode /nix

RUN addgroup --system nixbld
RUN adduser ${USER} root
RUN adduser ${USER} nixbld
RUN for i in $(seq 1 30); do useradd -ms /bin/bash nixbld$i &&  adduser nixbld$i nixbld; done

# without this line, if our docker has prebuild nix stuff, nix fails to install
RUN chmod --verbose -R a+rwx /nix

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


# RUN mkdir --parents /home/vscode/.nix-profile/etc/profile.d/
# #RUN cp /root/.nix-profile/etc/profile.d/nix.sh /home/vscode/.nix-profile/etc/profile.d/nix.sh
# RUN chmod +x /home/vscode/.nix-profile/etc/profile.d/nix.sh
# RUN chown --recursive vscode:vscode /home/vscode/.nix-profile/

# RUN echo "source /home/vscode/.nix-profile/etc/profile.d/nix.sh" >> /home/vscode/.bashrc && \
#     echo "source /home/vscode/.nix-profile/etc/profile.d/nix.sh" >> /home/vscode/.profile && \
#     echo "source /home/vscode/.nix-profile/etc/profile.d/nix.sh" >> /home/vscode/.bash_profile