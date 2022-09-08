# TODO: install hadolint via official nix, no need to get full docker image just to copy one file
FROM hadolint/hadolint:latest as hadolint

FROM composablefi/base-ci-linux:1.62.1

ARG NIGHTLY_VERSION=nightly-2022-08-06

RUN rustup toolchain install ${NIGHTLY_VERSION} && \
    rustup component add clippy && \
    rustup component add clippy --toolchain ${NIGHTLY_VERSION} && \
    rustup component add rustfmt && \
    rustup component add rustfmt --toolchain ${NIGHTLY_VERSION} && \
    rustup target install wasm32-unknown-unknown --toolchain ${NIGHTLY_VERSION} && \
    cargo +${NIGHTLY_VERSION} install -f cargo-llvm-cov --version 0.3.3 && \
    rustup component add llvm-tools-preview --toolchain=${NIGHTLY_VERSION} && \
    cargo install taplo-cli --version 0.6.10 && \
    cargo install cargo-spellcheck --version 0.11.3 && \
    cargo install mdbook --version 0.4.21 && \
    cargo install subxt-cli --version 0.22.0 && \
    cargo +${NIGHTLY_VERSION} install cargo-udeps --version 0.1.30 --locked && \
    ln -s "${RUSTUP_HOME}/toolchains/${NIGHTLY_VERSION}-x86_64-unknown-linux-gnu" "${RUSTUP_HOME}/toolchains/nightly-x86_64-unknown-linux-gnu" && \
    apt-get update && \
    apt-get install -y --no-install-recommends libfreetype6-dev libexpat1-dev && \
    apt-get autoremove -y && \
    apt-get clean && \
    find /var/lib/apt/lists/ -type f -not -name lock -delete

COPY --from=hadolint /bin/hadolint /usr/local/bin/
