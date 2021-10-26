FROM paritytech/ci-linux:production as builder
LABEL description="This is the build stage for Composable. Here we create the binary."

RUN apt-get update \
    && apt-get install -y --no-install-recommends apt-transport-https ca-certificates curl gnupg2 software-properties-common
SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN curl -fsSL https://download.docker.com/linux/debian/gpg | apt-key add - \
    && add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/debian $(lsb_release -cs) stable" \
    && apt-get update \
    && apt-get install -y --no-install-recommends cron docker-ce vim \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

USER root 

ARG VERSION=stable
ARG PROFILE=release
WORKDIR /composable

COPY . /composable
# please update docker, toolchain toml and github action with toolchain at same time
# there is no parity docker with same version, any later version breaks 3.0 parity (need to use 4.0)
# RUN rustup install ${VERSION}-x86_64-unknown-linux-gnu \
#     && rustup default $VERSION \
#     && rustup target add wasm32-unknown-unknown
# RUN pwd && ls && cd node && cargo build --$PROFILE

EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["cargo"]
