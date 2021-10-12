FROM paritytech/ci-linux:production as builder
LABEL description="This is the build stage for Composable. Here we create the binary."

RUN apt-get update
RUN apt-get -y install apt-transport-https ca-certificates curl gnupg2 software-properties-common
RUN curl -fsSL https://download.docker.com/linux/debian/gpg | apt-key add -
RUN add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/debian $(lsb_release -cs) stable"


RUN apt-get update \  
&& apt-get install cron -y && apt-get install docker-ce vim -y

USER root 

ARG VERSION=stable
ARG PROFILE=release
WORKDIR /composable

COPY . /composable

RUN chmod +x /composable/docker/build.sh
# please update docker, toolchain toml and github action with toolchain at same time
# there is no parity docker with same version, any later version breaks 3.0 parity (need to use 4.0)
RUN rustup install ${VERSION}-x86_64-unknown-linux-gnu
RUN rustup default $VERSION
RUN rustup target add wasm32-unknown-unknown
# RUN pwd && ls && cd node && cargo build --$PROFILE

EXPOSE 30333 9933 9944
VOLUME ["/data"]

RUN cargo build

CMD ["cargo"]