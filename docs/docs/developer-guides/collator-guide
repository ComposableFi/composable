# Collator Set-up Guide


This guide will show you how to set up a collator for Picasso. Please follow the steps below to complete the setup.

### Select (virtual) hardware

To run a collator, Composable recommends a minimum of 2 CPUs, 6 GB of memory, and 600GB of storage (will increase over time).

### Generate a new node key

First, you need to generate a temporary file containing both the node identity and node key.

```sh
sudo docker run --rm -ti -u$(id -u):$(id -g) parity/subkey generate-node-key > /tmp/tmp_not_a_real_key
```

Now retrieve the node identity. It will look something like "12D3KooWLUrUPDD93kTTdk4tFDrZmYLqnT5Ch9aG9A9gj6C7pv5M".

```sh
head -1 /tmp/tmp_not_a_real_key
```

The following command will split the node key to a separate file ( /tmp/not_a_real_key ).

```sh
tail -1 /tmp/tmp_not_a_real_key >/tmp/not_a_real_key
```

Copy the key file to the machine that will run the collator and place it in a safe location (In the example we use, the location is /home/composable/node-key).

### Run the collator

Use the following Docker Compose file to run the application.

```yml
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

An environment file can be used to pass in the two variables above (COMPOSABLE_FLAGS and COMPOSABLE_VERSION).  Alternatively, the values can be placed directly in the file.


```yaml
COMPOSABLE_FLAGS="--chain=picasso --name=partner-collator --listen-addr=/ip4/0.0.0.0/tcp/30334 --prometheus-external --prometheus-port 9615 --base-path /data --execution=wasm --collator --pruning=archive --node-key-file=/node-key -- --execution=wasm --listen-addr=/ip4/0.0.0.0/tcp/30333 "
COMPOSABLE_VERSION="v2.10009.0"
```

:::note
Please ensure that you modify the "name" parameter to a unique value that is specific to you.
:::

With the compose information in “docker-compose.yml” and the environment information in a file called “environment”, run the following command to start the application:

```sh
sudo apt-get install -y docker-compose
sudo docker-compose --env-file environment up -d
```

To see logs, run:

```sh
sudo docker logs -f $(sudo docker ps |grep composable|awk '{print $1}')
```

The latest version of the application can be found at [Docker hub] or [Composable's releases].

[Docker hub]: https://hub.docker.com/r/composablefi/composable/tags
[Composable's releases]: (https://github.com/ComposableFi/composable/releases/)

This configuration will pass the key into the Composable application at startup. Verify that it is being used by checking the log for:

```sh
[Parachain] 🏷 Local node identity is: 12D3KooWLp9aJBC7Jury1EgBYT4prqThmKPyB1Fm2fpBPUok1tKb
```

### Verify that the node is running and catching up
Go to [Polkadot Telemetry]. On Picasso's page, you should be able to see your node (based on the name you assigned it above), and verify that it is catching up.

[Polkadot Telemetry]: (https://telemetry.polkadot.io/#list/0x6811a339673c9daa897944dcdac99c6e2939cc88245ed21951a0a3c9a2be75bc)

### Verify that the http RPC port is available

```sh
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "rpc_methods"}' http://127.0.0.1:9933/
```

If this does not return anything, you may need to temporarily enable a few flags.

```yaml
--rpc-external \
--unsafe-rpc-external \
--rpc-methods=unsafe \
```

### Get collator session key

On each new collator, run:

```sh
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys" }' http://127.0.0.1:9933/
```

Use the resulting session key in the final step.

### Link node(s) to wallet(s)

On Polkadotjs, at https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpicasso-rpc.composable.finance#/explorer. 

Head to "Extrinsics" in the Developer tab. As the wallet account, run session setKeys(). Use the result from above as the Key and for the proof, enter "0x" as depicted in the example image below.

![polkadotjs_collator](./polkadotjs-collator.png)

:::note
Make sure you are running your node in collator mode, then provide the Public address of your collator wallet to Composable so it can be added as an approved collator.  

If you encounter difficulties during the setup process, please feel free to create a ticket on our Discord server for assistance.
:::
