# Picasso Collator Set-up Guide

In this document we will cover how to set up a collator with Composable Finance.
Please follow the steps below to get set up.

## 1) Select (virtual hardware)

To run a collator, Composable recommends a minimum of 2 CPUs, 6 GB of memory, 
and 600 GB of storage (will increase over time)

## 2) Generate a new node key

If you don’t have subkey installed, these steps will build it on an ubuntu machine:
```shell
sudo apt install --assume-yes git clang curl libssl-dev build-essential protobuf-codegen protobuf-compiler 


curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
#Choose -> 1) Proceed with installation (default)
source $HOME/.cargo/env
rustup default stable
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
rustup show
# Here we should see two toolchains installed; stable and nightly
rustup +nightly show
# Here we should see wasm32 and x86_64 installed targets


git clone --depth 1 https://github.com/paritytech/substrate.git
cd substrate


cargo +nightly build --package subkey --release
./target/release/subkey --help
```

For more details on subkey please see https://docs.substrate.io/reference/command-line-tools/subkey/

You can now use subkey to generate node keys for a node.
```shell
$ ./target/release/subkey generate-node-key --file /tmp/not_a_real_key
12D3KooWLp9aJBC7Jury1EgBYT4prqThmKPyB1Fm2fpBPUok1tKb
```
The file will contain the node key and the output( 

12D3KooWLp9aJBC7Jury1EgBYT4prqThmKPyB1Fm2fpBPUok1tKb

)above is the node identity.

Copy the key file to the machine that will run the collator and place it in a safe location. 

`/home/composable/node-key` is used in the example below.

## 3) Run the collator

Below is a docker compose file to run the application.
```yaml
services:
  composable_node:
    image: composablefi/composable:${COMPOSABLE_VERSION}
    container_name: composable_node
    volumes:
    - /var/lib/composable-data:/data
    - /home/composable/node-key:/node-key
    ports:
    - 9933:9933
    - 9944:9944
    - 30334:30334
    - 30333:30333
    - 9615:9615
    restart: unless-stopped
    command:  >
      /bin/composable ${COMPOSABLE_FLAGS}
    networks:
    - composable_network




networks:
  composable_network:
    name: composable_network
    driver: bridge
```
An environment file can be used to pass in the two variables above (COMPOSABLE_FLAGS and COMPOSABLE_VERSION).  

Alternatively, the values can be placed directly in the file.
```yaml
COMPOSABLE_FLAGS="--chain=picasso --name=partner-collator --listen-addr=/ip4/0.0.0.0/tcp/30334 --prometheus-external --prometheus-port 9615 --base-path /data --execution=wasm --collator --pruning=archive --node-key-file=/node-key -- --execution=wasm --listen-addr=/ip4/0.0.0.0/tcp/30333 "
COMPOSABLE_VERSION="v2.10009.0"
```
**Make sure to change the name parameter to be something unique to you.**

With the compose information in "docker-compose.yml" and the environment information in a file called "environment",

run the following command to start the application:
```shell
sudo apt-get install -y docker-compose
sudo docker-compose --env-file environment up -d
```
To see logs:

```shell
sudo docker logs -f $(sudo docker ps |grep composable|awk '{print $1}')
```
The latest version of the application can be found at  
https://hub.docker.com/r/composablefi/composable/tags or https://github.com/ComposableFi/composable/releases/.

This configuration will pass the key into the Composable application at startup. 
Verify that it is being used by checking the log for

```shell
[Parachain] 🏷 Local node identity is: 12D3KooWLp9aJBC7Jury1EgBYT4prqThmKPyB1Fm2fpBPUok1tKb
```

## 4) Verify that the node is running and catching up

Go to https://telemetry.polkadot.io/#list/0x6811a339673c9daa897944dcdac99c6e2939cc88245ed21951a0a3c9a2be75bc

From that site, you should be able to see your node, based on the name you assigned it above, 
and verify that it is catching up.

## 5) Verify that the http RPC port is available

```shell
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "rpc_methods"}' http://127.0.0.1:9933/
```
If this does not return anything, you may need to temporarily enable a few flags

```yaml
--rpc-external \
--unsafe-rpc-external \
--rpc-methods=unsafe \
```

## 6) Collator session key

On each new collator, run:
```shell
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys" }' http://127.0.0.1:9933/
```
Use the resulting session key in the next step.

## 7) Link node(s) to wallet(s)

Go to polkadot js, at https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpicasso-rpc.composable.finance#/explorer. 

Go to "Developer" ➝ "extrinsics". As the wallet account, run session setKeys(). 

Use the result from above as the Key and for the proof, enter "0x"

![proof](./images-picasso-collator-setup/proof.png)

Make sure you are running your node in collator mode, then provide the Public address of your collator wallet to 
Composable so it can be added as an approved collator.