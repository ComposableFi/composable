FROM rust:1.55 as builder

ADD init.sh .

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata git acl packer \
    && apt-get install -y jq libssl-dev build-essential python3-pip \
    libclang-dev curl clang zip cmake curl pkg-config cmake ansible \
    && rm -rf /var/lib/apt/lists/*

RUN sh ./init.sh

CMD [cargo]
