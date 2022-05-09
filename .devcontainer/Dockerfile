ARG BASEDEV_VERSION=v0.12.0

# ARG RUST_SRC_PATH = "${rust-nightly}/lib/rustlib/src/rust/src"
# ARG LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib"
# ARG PROTOC = "${protobuf}/bin/protoc"
# ARG ROCKSDB_LIB_DIR = "${rocksdb}/lib"

FROM qmcgaw/basedevcontainer:${BASEDEV_VERSION}-debian
ARG CREATED
ARG COMMIT
ARG VERSION=local
LABEL \
    org.opencontainers.image.authors="quentin.mcgaw@gmail.com" \
    org.opencontainers.image.created=$CREATED \
    org.opencontainers.image.version=$VERSION \
    org.opencontainers.image.revision=$COMMIT \
    org.opencontainers.image.url="https://github.com/qdm12/rustdevcontainer" \
    org.opencontainers.image.documentation="https://github.com/qdm12/rustdevcontainer" \
    org.opencontainers.image.source="https://github.com/qdm12/rustdevcontainer" \
    org.opencontainers.image.title="Rust Debian Dev container" \
    org.opencontainers.image.description="Rust development container for Visual Studio Code Remote Containers development"
WORKDIR /workspace

# Install Rust for the correct CPU architecture
ARG RUST_VERSION=1.60.0
ARG RUSTUP_INIT_VERSION=1.24.3
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH
RUN arch="$(uname -m)" && \
    case "$arch" in \
    x86_64) rustArch='x86_64-unknown-linux-gnu'; rustupSha256='3dc5ef50861ee18657f9db2eeb7392f9c2a6c95c90ab41e45ab4ca71476b4338' ;; \
    aarch64) rustArch='aarch64-unknown-linux-gnu'; rustupSha256='32a1532f7cef072a667bac53f1a5542c99666c4071af0c9549795bbdb2069ec1' ;; \
    *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac && \
    wget -qO /tmp/rustup-init "https://static.rust-lang.org/rustup/archive/${RUSTUP_INIT_VERSION}/${rustArch}/rustup-init" && \
    echo "${rustupSha256}  /tmp/rustup-init" | sha256sum -c - && \
    chmod +x /tmp/rustup-init && \
    /tmp/rustup-init -y --no-modify-path --default-host ${rustArch} && \
    rm /tmp/rustup-init && \
    chmod -R a+w ${RUSTUP_HOME} ${CARGO_HOME}

# Install:
# - gcc, libc6-dev required by Rust
# - musl-tools required for static binaries
# TODO: openssl-devel on Fedora
RUN export DEBIAN_FRONTEND=noninteractive && \
    apt-get update && \
    apt-get -y install --no-install-recommends clang libclang-dev gcc libc6-dev musl-tools make openssl libssl-dev pkg-config && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
RUN ln -s /usr/bin/gcc /usr/bin/"$(uname -m)"-linux-musl-gcc

ARG LIBCLANG_PATH="$(sudo find / -name libclang.so)"

RUN echo whereis c++

RUN rustup update stable && \
    rustup update nightly-2022-04-18 && \
    rustup target add wasm32-unknown-unknown --toolchain nightly-2022-04-18 && \
    rustup default stable

# Install Rust tooling
ARG RUST_ANALYZER_VERSION=2022-04-25
RUN wget -qO- "https://github.com/rust-analyzer/rust-analyzer/releases/download/${RUST_ANALYZER_VERSION}/rust-analyzer-$(uname -m)-unknown-linux-gnu.gz" | \
    gunzip > /usr/local/bin/rust-analyzer && \
    chmod 700 /usr/local/bin/rust-analyzer

RUN rustup component add clippy && \
    rustup component add rustfmt && \
    rustup component add rust-src