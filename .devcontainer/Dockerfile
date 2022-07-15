# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.233.0/containers/rust/.devcontainer/base.Dockerfile

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="buster"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT}

# [Optional] Uncomment this section to install additional packages.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends clang libclang-dev gcc libc6-dev musl-tools openssl libssl-dev pkg-config

# Configure rust with corect channels and targets. 
RUN rustup update stable && \
    rustup update nightly-2022-04-18 && \
    rustup target add wasm32-unknown-unknown --toolchain nightly-2022-04-18 && \
    rustup default stable && \
    rustup component add clippy && \
    rustup component add rustfmt && \
    rustup component add rust-src && \
    rustup component add rustfmt --toolchain nightly-2022-04-18

# Install rust-analyzer
ARG RUST_ANALYZER_VERSION=2022-04-25
RUN wget -qO- "https://github.com/rust-analyzer/rust-analyzer/releases/download/${RUST_ANALYZER_VERSION}/rust-analyzer-$(uname -m)-unknown-linux-gnu.gz" | \
    gunzip > /usr/local/bin/rust-analyzer && \
    chmod 700 /usr/local/bin/rust-analyzer

# # Add polkadot
# Not straight to the workspace dir because that seems to be causing issues.
# We move it in the devcontainer.json's postCreateCommand.
WORKDIR /polkadot-binary
RUN wget "https://github.com/paritytech/polkadot/releases/download/v0.9.24/polkadot" && \
    chmod +x polkadot


# # Add basilisk node
WORKDIR /basilisk-node
RUN wget "https://github.com/galacticcouncil/Basilisk-node/releases/download/v8.0.0/basilisk" && \
    chmod +x basilisk

# Add mdbook
WORKDIR /mdbook-binary
RUN wget "https://github.com/rust-lang/mdBook/releases/download/v0.4.18/mdbook-v0.4.18-x86_64-unknown-linux-gnu.tar.gz" && \ 
    tar -xzf mdbook-v0.4.18-x86_64-unknown-linux-gnu.tar.gz && \
    chmod +x mdbook && \
    cp ./mdbook /usr/bin 

# Add taplo
WORKDIR /taplo-binary
RUN wget "https://github.com/tamasfe/taplo/releases/download/release-cli-0.6.2/taplo-0.6.2-x86_64-unknown-linux-gnu.tar.gz" && \
    tar -xzf taplo-0.6.2-x86_64-unknown-linux-gnu.tar.gz && \
    chmod +x taplo && \
    cp ./taplo /usr/bin 

# Add btm
WORKDIR /btm-binary
RUN wget "https://github.com/ClementTsang/bottom/releases/download/0.6.8/bottom_x86_64-unknown-linux-gnu.tar.gz" && \
    tar -xzf bottom_x86_64-unknown-linux-gnu.tar.gz && \
    chmod +x btm && \
    cp ./btm /usr/bin 
