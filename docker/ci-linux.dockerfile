FROM hadolint/hadolint:latest as hadolint
FROM paritytech/ci-linux:production
LABEL summary="Image for Composable related CI and builds."

ARG NIGHTLY_VERSION=nightly-2021-11-29
# uncomment allow for in place/gradual upgrades, and add that on top of this
# ARG NEW_NIGHTLY_VERSION=nightly-2021-11-29

# do not unnstall nightly from parity, do not overoptimize
# that can be used to test issues on our image vs official

RUN rustup toolchain install ${NIGHTLY_VERSION} && \
    ln -s "${RUSTUP_HOME}/toolchains/${NIGHTLY_VERSION}-x86_64-unknown-linux-gnu" "${RUSTUP_HOME}/toolchains/nightly-x86_64-unknown-linux-gnu" && \
    rustup component add clippy && \
    rustup component add clippy --toolchain ${NIGHTLY_VERSION} && \
    rustup component add rustfmt && \
    rustup component add rustfmt --toolchain ${NIGHTLY_VERSION} && \
    rustup target install wasm32-unknown-unknown --toolchain ${NIGHTLY_VERSION} && \
    cargo +${NIGHTLY_VERSION} install --force cargo-llvm-cov && \
    rustup component add llvm-tools-preview --toolchain=${NIGHTLY_VERSION} && \
    cargo install --force taplo-cli && \
    cargo +${NIGHTLY_VERSION} install cargo-udeps --locked && \
    ln --symbolic "${RUSTUP_HOME}/toolchains/${NIGHTLY_VERSION}-x86_64-unknown-linux-gnu" "${RUSTUP_HOME}/toolchains/nightly-x86_64-unknown-linux-gnu" && \
    apt-get update && \
    apt-get install -y --no-install-recommends libfreetype6-dev libexpat1-dev diffutils

COPY --from=hadolint /bin/hadolint /usr/local/bin/
