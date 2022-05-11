# Collator Setup Guide

In this document we will cover how to set up a collator with Composable Finance. 
There are several ways to do that.

* Build node locally

* Download pre-build node from GitHub releases

* Run node in docker

## Build from source

In this step we will set up a rust compiler, toolchain and build a node.

Setup required libraries

```sh
#!/bin/bash

RUST_C="nightly-2022-04-18"

curl https://sh.rustup.rs -sSf | sh -s -- -y && \
export PATH="$PATH:$HOME/.cargo/bin" && \
rustup toolchain uninstall $(rustup toolchain list) && \
rustup toolchain install $RUST_C && \
rustup target add wasm32-unknown-unknown --toolchain $RUST_C && \
rustup default $RUST_C && \
rustup show
```

Get project and build node

```sh
git clone --depth 1 --branch v2.1.6 https://github.com/ComposableFi/composable.git && \
cd composable && \
export SKIP_WASM_BUILD=1 && \
cargo build --release 
```

One-liner

```sh
RUST_C="nightly-2022-04-18"
RELEASE_TAG="v2.1.6"

sudo apt install -y git clang curl libssl-dev llvm libudev-dev && \
git clone --depth 1 --branch $RELEASE_TAG https://github.com/ComposableFi/composable.git && \
cd composable && \
curl https://sh.rustup.rs -sSf | sh -s -- -y && \
export PATH="$PATH:$HOME/.cargo/bin" && \
rustup toolchain uninstall $(rustup toolchain list) && \
rustup toolchain install $RUST_C && \
rustup target add wasm32-unknown-unknown --toolchain $RUST_C && \
rustup default $RUST_C && \
rustup show && \
export SKIP_WASM_BUILD=1 && \
cargo build --release
```

The compiled node should be in

```sh
./target/release
```

## Run as systemd service

### Download the prebuilt node

```sh
wget https://github.com/ComposableFi/composable/releases/download/v2.1.6/composable
```

Put compiled binary to `/usr/bin/`

```sh
cp ./target/release/composable /usr/bin
```

Create `collator.service` file and put following content into it. Save it with `Ctrl+O`

```sh
sudo nano /etc/systemd/system/collator.service
```

Enable and Start service

```sh
sudo systemctl enable collator.service
sudo systemctl start collator.service
```

Check service status

```sh
sudo systemctl status collator.service
```

Check logs output
```sh
journalctl -f
```

collator.service content

```ini
[Unit]
Description=Composable

[Service]

ExecStart=/usr/local/bin/сomposable \
--collator \
--chain=picasso \
--pruning=archive \
--base-path /var/lib/composable-data/ \
--port 30333 \
--listen-addr=/ip4/0.0.0.0/tcp/30334 \
--execution wasm \
-- \
--execution=wasm \
--pruning=archive \
--listen-addr=/ip4/0.0.0.0/tcp/30333


# (file size)
LimitFSIZE=infinity
# (cpu time)
LimitCPU=infinity
# (virtual memory size)
LimitAS=infinity
# (locked-in-memory size)
LimitMEMLOCK=infinity
# (open files)
LimitNOFILE=64000
# (processes/threads)
LimitNPROC=64000

Restart=always
RestartSec=120

[Install]
WantedBy=multi-user.target
```

## Setup docker and docker-compose

```sh
sudo apt install apt-transport-https ca-certificates curl gnupg-agent software-properties-common 
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
sudo add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable"
sudo apt install docker-ce docker-ce-cli containerd.io
```

Optional steps

```sh
sudo apt-mark hold docker-ce # prevent the Docker package from being updated, so no sudden updates and process interuption
sudo usermod -aG docker $USER # adds docker to sudo group so there's no need to run it from root
```

Setup docker-compose

```
sudo curl -L "https://github.com/docker/compose/releases/download/1.29.2/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose

sudo chmod +x /usr/local/bin/docker-compose
```

Check docker installation

```sh
sudo systemctl status docker
docker container run hello-world
```

### docker-compose

Save the following content to `docker-compose.yml` and run `docker-compose up -d`

```yml
version: "3.7"

services:
  composable_node:
    image: composablefi/composable:latest
    container_name: composable_node
    volumes:
      - ./chain-data:/chain-data
    ports:
      - 9833:9833
      - 9844:9844
      - 40333:40333
      - 30333:30333
    restart: unless-stopped
    command: >
      /usr/local/bin/сomposable
      --collator 
      --chain=picasso
      --pruning=archive
      --base-path /chain-data
      --port 30333
      --unsafe-ws-external
      --unsafe-rpc-external
      --listen-addr=/ip4/0.0.0.0/tcp/30334k
      --execution wasm
      --
      --execution=wasm 
      --listen-addr=/ip4/0.0.0.0/tcp/30333
```

### How to start the Composable node with Granfana

```sh
cd docker-compose-collator/

docker-compose up 

```

Prometheus should be available on `127.0.0.1:9090`

Cadvisor `127.0.0.1:8080`

Grafana `127.0.0.1:3000`

Default login/password is `admin`/`admin123`

To make grafana work, prometheus needs to be added as data source 

![grafana-1](./grafana-1.png)

![grafana-2](./grafana-2.png)

![grafana-3](./grafana-3.png)

![grafana-4](./grafana-4.png)

Import dashboard

![grafana-5](./grafana-5.png)

![grafana-6](./grafana-6.png)

Choose prometheus source

![grafana-7](./grafana-7.png)

![grafana-8](./grafana-8.png)

