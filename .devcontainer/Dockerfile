# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.234.0/containers/rust/.devcontainer/base.Dockerfile

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT}

# [Optional] Uncomment this section to install additional packages.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends libssl-dev clang lld libclang-dev make cmake \
    git pkg-config curl time ca-certificates \
    xz-utils unzip 

RUN rustup update stable && rustup update nightly-2022-04-18 && rustup target add wasm32-unknown-unknown --toolchain nightly-2022-04-18 && rustup default stable

