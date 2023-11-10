# Composable Cosmos Testnet 4 

Composable Cosmos Testnet 4 serves the following purposes:

- Testing chain upgrades before mainnet releases.
- Connecting Ethereum Goerli to the first Cosmos chain via IBC.
- An environment for developers and users to test the functionalities of CVM and MANTIS on a Cosmos-only blockchain.

## Information
- Network information: https://github.com/notional-labs/composable-networks/tree/main/banksy-testnet-4
- Chain ID: banksy-testnet-4
- Genesis: https://raw.githubusercontent.com/notional-labs/composable-networks/main/banksy-testnet-4/genesis.json
- Binary: https://github.com/notional-labs/composable-centauri/releases/tag/v5.2.5-testnet4
- Current version: v5.2.5-testnet4
- Peers: a89d3d9fc0465615aa1100dcf53172814aa2b8cf@168.119.91.22:2260
- Public Notional endpoints:
  - RPC: https://rpc-banksy4.notional.ventures:443
  - API: https://api-banksy4.notional.ventures:443
  - gRPC: http://168.119.91.22:2263
- Block Explorer: https://explorer.stavr.tech/Composable-Testnet4
- Faucet: [Composable Discord](https://discord.com/invite/composable)

## Setup Instruction

**1. Building the binary**

There are two ways of setting up the `centaurid` binary, building from source or installing from binary URL, which is mentioned in the install script in the next section.

To build the binary from source, run these commands:

```
#mkdir $HOME/go/bin # ignore this command if you already have $HOME/go/bin folder
export PATH=$PATH:$HOME/go/bin
cd $HOME
git clone https://github.com/notional-labs/composable-centauri
cd composable-centauri
git checkout v5.2.5-testnet4 # Using v5.2.5-testnet4
make install
centaurid version # v5.2.5-testnet4
```

**2. Joining testnet**

Here is a full script to install `centaurid binary` and run the node with state sync. This script should be run with administration privileges by running `sudo script.sh`:

```
# script.sh
centaurid init <moniker> --chain-id banksy-testnet-4
wget https://raw.githubusercontent.com/notional-labs/composable-networks/main/banksy-testnet-4/genesis.json -O $HOME/.banksy/config/genesis.json


# state sync
## downloading wasm snapshot first
curl -o - -L https://composable.wasmt4.stavr.tech/wasm-composable.tar.lz4 | lz4 -c -d - | tar -x -C $HOME/.banksy --strip-components 2

SNAP_RPC="https://rpc-banksy4.notional.ventures:443"
LATEST_HEIGHT=$(curl -s $SNAP_RPC/block | jq -r .result.block.header.height); \
BLOCK_HEIGHT=$((LATEST_HEIGHT - 2000)); \
TRUST_HASH=$(curl -s "$SNAP_RPC/block?height=$BLOCK_HEIGHT" | jq -r .result.block_id.hash)
sed -i.bak -E "s|^(enable[[:space:]]+=[[:space:]]+).*$|\1true| ; \
s|^(rpc_servers[[:space:]]+=[[:space:]]+).*$|\1\"$SNAP_RPC,$SNAP_RPC\"| ; \
s|^(trust_height[[:space:]]+=[[:space:]]+).*$|\1$BLOCK_HEIGHT| ; \
s|^(trust_hash[[:space:]]+=[[:space:]]+).*$|\1\"$TRUST_HASH\"|" $HOME/.banksy/config/config.toml

# run node
centaurid start --p2p.seeds a89d3d9fc0465615aa1100dcf53172814aa2b8cf@168.119.91.22:2260
```

**3. Join testnet as a validator**
To join `banksy-testnet-4` as a validator, you should setup a running node as in step 1 above and wait for it to be fully synced, and then setup validator:

- Create a key:
```
centaurid keys add <validator-key> # generate a new key, or use `--recover` to recover an existed key with mnemonic
```

- For testnet tokens, head to ethibc-testnet-faucet channel on the Composable discord and send the following message with your address included:

```
$request <address> composable
```

- To see the current balance of the address, run the following command:

```
$balance <address> composable
```

- Create validator:
```
centaurid tx staking create-validator --amount=1000000000000ppica --moniker="<validator-name>" --chain-id=banksy-testnet-4 --commission-rate="0.05"  --commission-max-change-rate="0.01" --commission-max-rate="0.20" --from=<validator-key> --node=https://rpc-banksy4.notional.ventures:443 --gas=auto --min-self-delegation 10 --pubkey=$(centaurid tendermint show-validator)
```

To add validator info:

```
centaurid tx staking edit-validator \
    --website="" \ # URL to validator website
    --identity="" \ # keybase.io identity 
    --details="" \ # Additional detail 
    --security-contact="" \ # security email
    --from=<validator-key> \
    --node="https://rpc-banksy4.notional.ventures:443"
```
