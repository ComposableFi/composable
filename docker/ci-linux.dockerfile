FROM paritytech/ci-linux:production

RUN rustup toolchain uninstall nightly-2021-11-08 && \
    rustup toolchain uninstall nightly && \
    rustup toolchain install nightly-2021-11-29 && \
    rustup target install wasm32-unknown-unknown --toolchain nightly-2021-11-29 && \
    ln -s "${RUSTUP_HOME}/toolchains/nightly-2021-11-29-x86_64-unknown-linux-gnu" "${RUSTUP_HOME}/toolchains/nightly-x86_64-unknown-linux-gnu"
