FROM paritytech/ci-linux:production as builder
LABEL description="This is the build stage for Composable. Here we create the binary."

USER root 

ARG VERSION=stable
ARG PROFILE=release
WORKDIR /composable

COPY . /composable
# please update docker, toolchain toml and github action with toolchain at same time
# there is no parity docker with same version, any later version breaks 3.0 parity (need to use 4.0)
RUN rustup install ${VERSION}-x86_64-unknown-linux-gnu
RUN rustup default $VERSION
RUN rustup target add wasm32-unknown-unknown
# RUN pwd && ls && cd node && cargo build --$PROFILE

EXPOSE 30333 9933 9944
VOLUME ["/data"]


# ENTRYPOINT ./docker/init.sh
CMD ["cargo", "build", "release"]
