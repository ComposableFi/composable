FROM hadolint/hadolint:latest as hadolint

FROM paritytech/ci-linux:production

# allow for in place/gradual upgrades
ARG NIGHTLY_VERSION=nightly-2021-11-29
ARG NEW_NIGHTLY_VERSION=nightly-2021-11-29

# do not unnstall nightly from parity, do not overoptimize
# that can be used to test issues on our image vs official 

RUN rustup toolchain install ${NIGHTLY_VERSION} && \
    #as in https://github.com/paritytech/scripts/blob/master/dockerfiles/ci-linux/Dockerfile
    ln -s /usr/local/rustup/toolchains/ ${NIGHTLY_VERSION}-x86_64-unknown-linux-gnu /usr/local/rustup/toolchains/nightly-x86_64-unknown-linux-gnu && \
    rustup component add clippy && \
    rustup component add clippy --toolchain ${NIGHTLY_VERSION} && \
    rustup component add rustfmt && \
    rustup component add rustfmt --toolchain ${NIGHTLY_VERSION} && \
    rustup target install wasm32-unknown-unknown --toolchain ${NIGHTLY_VERSION} && \
    cargo +${NIGHTLY_VERSION} install -f cargo-llvm-cov && \
    rustup component add llvm-tools-preview --toolchain=${NIGHTLY_VERSION} && \
    cargo install taplo-cli && \
    cargo +${NIGHTLY_VERSION} install cargo-udeps --locked && \
    ln -s "${RUSTUP_HOME}/toolchains/${NIGHTLY_VERSION}-x86_64-unknown-linux-gnu" "${RUSTUP_HOME}/toolchains/nightly-x86_64-unknown-linux-gnu"

RUN  apt-get update && \
    apt-get install -y --no-install-recommends libfreetype6-dev libexpat1-dev

COPY --from=hadolint /bin/hadolint /usr/local/bin/
