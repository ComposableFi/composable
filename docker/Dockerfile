FROM composablefi/rust:latest
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

ARG VERSION=stable
ARG AWS_ACCESS_KEY_ID=${AWS_ACCESS_KEY_ID}
ARG AWS_SECRET_ACCESS_KEY=${AWS_SECRET_ACCESS_KEY}
ARG PROFILE=release
WORKDIR /composable

COPY . /composable
# please update docker, toolchain toml and github action with toolchain at same time
# there is no parity docker with same version, any later version breaks 3.0 parity (need to use 4.0)
RUN rustup install ${VERSION}-x86_64-unknown-linux-gnu \
    && rustup default $VERSION \
    && rustup target add wasm32-unknown-unknown
# RUN pwd && ls && cd node && cargo build --$PROFILE

RUN cargo build --release

RUN cp -R /composable/target/release /usr/local/bin




### STAGE 2 ###
# FROM debian:buster-slim
# LABEL description="This is the 2nd stage: a very small image where we copy the Composable binary."
# ARG PROFILE=release
# COPY --from=builder /composable/target/$PROFILE/node-template /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /composable composable && \
	mkdir -p /composable/.local/share && \
	mkdir /data && \
	chown -R composable:composable /data 

USER composable
EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["cargo"]
