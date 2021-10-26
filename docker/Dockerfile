FROM composablefi/rust:latest
LABEL description="This is the build stage for Composable. Here we create the binary."

RUN apt-get update
RUN apt-get -y install apt-transport-https ca-certificates curl gnupg2 software-properties-common automake libtool inotify-tools autoconf
RUN curl -fsSL https://download.docker.com/linux/debian/gpg | apt-key add -
RUN add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/debian $(lsb_release -cs) stable"


RUN apt-get update \  
&& apt-get install cron -y && apt-get install docker-ce vim -y

ARG VERSION=stable
ARG AWS_ACCESS_KEY_ID=${AWS_ACCESS_KEY_ID}
ARG AWS_SECRET_ACCESS_KEY=${AWS_SECRET_ACCESS_KEY}
ARG PROFILE=release
WORKDIR /composable

COPY . /composable

RUN cargo build --release


RUN useradd -m -u 1000 -U -s /bin/sh -d /composable composable && \
	mkdir -p /composable/.local/share && \
	mkdir /data && \
	chown -R composable:composable /data 

USER composable
EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["/composable/target/release"]