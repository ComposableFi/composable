# Walkthrough: cw20_base

In this walkthrough, we will upload and interact with a `cw20_base` contract on a local Picasso network by:

* Fetching the contract binary from a running Cosmos chain and upload it to our chain.
* Instantiating the contract.
* Executing a transfer.

:::note
Ensure that you have followed the guide to setup your development environment as outlined in the [first section](https://docs.composable.finance/developer-guides/cosmwasm-cli).
:::

## Running `pallet-cosmwasm` on Picasso locally

**When running Picasso Rococo on a local development network versus on the Picasso Rococo mainnet, the only difference is that you need to replace '-n Alice' with your seed phrase in the commands and change the RPC endpoints. It is also required to add your port after the Rococo RPC endpoint, e.g. `wss://picasso-rococo-rpc-lb.composablenodes.tech:<insert port>`.**

### Uploading the contract

Let's say that we want to upload the `v1.0.1` release of `cw20_base`. We can directly use the download link from the [release page](https://github.com/CosmWasm/cw-plus/releases).

```sh
ccw substrate -c ws://127.0.0.1:9988 -n alice tx upload --url https://github.com/CosmWasm/cw-plus/releases/download/v1.0.1/cw20_base.wasm
```

Output:

```
[ + ] Contract uploaded.
    - Code Hash: 0x12f7…1f73
        - Code ID: 1
```

### Getting JSON output instead of plain text

Sometimes it is easy to get the output in JSON to automize the process. You can do that by using `--output-type` parameter.

```sh
ccw substrate --output-type json COMMAND
```

### Instantiating the contract

The next step is to instantiate the contract so that we have an instance of the contract that we can execute and query. The upload command returned a code ID. This code ID is used to identify the wasm binary (compiled CosmWasm contract). We will use this code ID to instantiate the contract from.

We want to use the following configurations to instantiate the contract:

- Code ID: `1` which is returned from the previous upload command.
- Salt: Just a random salt.
- Label: Let's say that it is "our-fancy-cw20base-contract".
- Maximum gas: We don't care, let's set it to `10000000000`
- Instantiate message: Let's instantiate a PICA token and give some initial balance to Bob's account
```json
{
  "name": "Picasso",
  "symbol": "PICA",
  "decimals": 6,
  "initial_balances": [
    {
      "amount": "1000000000000",
      "address": "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"
    }
  ]
}
```

Note that in the instantiate message, we used a hexadecimal address. That is the hexadecimal representation of the ordinary SS58 public key. We will soon
make use of SS58 representations in the contracts as well, but for now, this is how we do it.

So the command will be:
```sh
ccw substrate -c ws://127.0.0.1:9988 -n alice \
    tx instantiate \
    -c 1 \
    -s random-salt \
    -l our-fancy-cw20base-contract \
    -g 10000000000 \
    -m '{"name":"Picasso","symbol":"PICA","decimals":6,"initial_balances":[{"amount":"1000000000000","address":"0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"}]}'
```

Note that apart from the address of the contract that is instantiated, we also get the raw CosmWasm events.

Output:
```
[ + ] Contract instantiated.
    - Contract address: 5EszMeNDPmy4orcLEHRLiJawAt5xAvfK5VH7REV8bpB1jtjX
- Event: instantiate
    - Contract: 5EszMeNDPmy4orcLEHRLiJawAt5xAvfK5VH7REV8bpB1jtjX
    - Attributes:
        - _contract_address: 0x7c888b21f31f0cec5149830bb0f8f20b949e2b589c240d223625f53df694ca49
        - code_id: 1
```

### Execute a transfer

Let's transfer some amount from `Bob` to `Charlie`.

Note that as the signer, we need to use `bob` instead of `alice` because the signer here will be both the `signer` of the extrinsic, and the `sender`
of the contract message. Since we want to transfer from `bob`, `bob` needs to be the caller of the `execute` call.

Also, the contract address used here is `5EszMeNDPmy4orcLEHRLiJawAt5xAvfK5VH7REV8bpB1jtjX` which should be the same for you as well since the algorithm for the address
generation that we use is based on the instantiate parameters that we provide, not on some random values or chain state. But if for some reason, you get a different
address, use that address to execute the contract.

```sh
ccw substrate -c ws://127.0.0.1:9988 -n bob \
    tx execute \
    -c  5EszMeNDPmy4orcLEHRLiJawAt5xAvfK5VH7REV8bpB1jtjX \
    -g 10000000000 \
    -m '{"transfer":{"amount":"1000","recipient":"0xd64439add16b49b6b68ac74e1b28a73a8491501ab7e0e829716f580947a4bd7e"}}'
```

Output:
```
[ + ] Contract executed.
- Event: execute
    - Contract: 5EszMeNDPmy4orcLEHRLiJawAt5xAvfK5VH7REV8bpB1jtjX
    - Attributes:
          - _contract_address: 0x7c888b21f31f0cec5149830bb0f8f20b949e2b589c240d223625f53df694ca49
- Event: wasm
    - Contract: 5EszMeNDPmy4orcLEHRLiJawAt5xAvfK5VH7REV8bpB1jtjX
    - Attributes:
        - action: transfer
        - from: 0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48
        - to: 0xd64439add16b49b6b68ac74e1b28a73a8491501ab7e0e829716f580947a4bd7e
        - amount: 1000
        - _contract_address: 0x7c888b21f31f0cec5149830bb0f8f20b949e2b589c240d223625f53df694ca49
```
  
  
### Query the balance

Although you can see that the events clearly show the transfer happened. Let's query the contract to check out our balance to make sure. Since the query is not a transaction
but an RPC call, we'll use the subcommand `rpc` instead of `tx`.

Note that we are using a different protocol and port for the RPC endpoint.

```
ccw substrate -c http://127.0.0.1:32200 \
    rpc query \
    -c 5EszMeNDPmy4orcLEHRLiJawAt5xAvfK5VH7REV8bpB1jtjX \
    -g 10000000000 \
    -q '{"balance":{"address":"0xd64439add16b49b6b68ac74e1b28a73a8491501ab7e0e829716f580947a4bd7e"}}'
```

Output:
```
ccw substrate -c http://127.0.0.1:32200 \
    rpc query \
    -c 5EszMeNDPmy4orcLEHRLiJawAt5xAvfK5VH7REV8bpB1jtjX \
    -g 100000000 \
    -q '{"balance":{"address":"0xd64439add16b49b6b68ac74e1b28a73a8491501ab7e0e829716f580947a4bd7e"}}'
```

## Running `pallet-cosmwasm` on Picasso Rococo

This is a specific example guide to upload, initialize and execute cw20 contracts on Picasso Rococo. The previous walkthough can also be applied to Picasso Rococo too, however, we have added another example specific to Rococo.


```sh
# Download the contract
curl --location https://github.com/CosmWasm/cw-plus/releases/download/v1.0.1/cw20_base.wasm > cw20_base.wasm`
```
```sh
# Upload the contract 
cargo run substrate -c wss://picasso-rococo-rpc-lb.composablenodes.tech:443 --seed "<SEED>" tx upload --file-path ./cw20_base.wasm 
```
```sh
# Instantiate the contract* 
cargo run substrate -c wss://picasso-rococo-rpc-lb.composablenodes.tech:443 --seed "<SEED>" tx instantiate --code-id 1 --salt 0x1234 --label 0x4321 --gas 10000000000 --message '{ "decimals" : 18, "initial_balances": [], "name" : "SHIB", "symbol" : "SHIB", "mint": {"minter" : "5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL"} }'
```
```sh
# Execute the contract
cargo run substrate -c wss://picasso-rococo-rpc-lb.composablenodes.tech:443 --seed "<SEED>" tx execute --contract "5CntM2NFn4Vkyu77tMDm5TRosKd9qskYpafh8L6Lz2FGP2rD" --gas 10000000000 --message '{ "mint" : { "amount" : "123456789", "recipient" : "5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL" }}'
```
```sh
# Query the contract
cargo run substrate -c wss://picasso-rococo-rpc-lb.composablenodes.tech:443 rpc query --contract "5CntM2NFn4Vkyu77tMDm5TRosKd9qskYpafh8L6Lz2FGP2rD" --gas 10000000000 --query '{"balance": {"address": "5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL"}}'
```

*After uploading the contract, please note that the "contract address" provided in the example of instantiating the contract may differ. It is possible that someone has already tested this smart contract on Picasso Rococo and uploaded it to the chain. As a result, you won't be able to upload the same contract again.

If you are running this contract locally, follow these steps:

1. Go to the 'Chain state' section within the 'Developer' tab.
2. Change the 'selected state query' to 'cosmwasm'.
3. Modify the dropdown option from 'codeHashTold' to 'contractToInfo'.
4. Toggle the 'include option' off.
5. This will retrieve the correct contract address. Refer to the image below as an example.

![polkadot_js1](./cw-cli.png)
Please ensure you follow these instructions to obtain the accurate contract address when running the contract locally.
