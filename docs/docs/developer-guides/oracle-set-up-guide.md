# Apollo Oracle Set-up Guide 


## Overview

Apollo is an oracle for submitting prices on-chain. 
We plan to upgrade it and allow everyone to stream arbitrary third party data in the future. 
By taking part in Apollo and becoming an oracle operator, 
you will be securing the assets prices and help the network become more resilient. 
Participants receive tokens as rewards for their work, 
but it is important to note that the on-chain algorithm also allows for slashing based on bad behavior, 
which is defined as submitting prices out of a certain threshold range from a pivot price calculated on all submitted prices.

Apollo consists of three key components. The On-chain pallet/worker that decides when a price is requested. 
An Off-Chain worker that monitors when a price has been requested and if so, submits a price for the request. 
Finally, a Price-Feed (we provide a reference implementation for this component) that fetches the prices from a CEX/DEX 
and caches them, such that the Off-Chain worker is able to query those prices and stream them by submitting a transaction. 
Below is a high level diagram that shows the interactions between the components.


## High level architecture


![oracle_architecture](../../static/img/developer-guides/oracle/architecture.jpg)


1. Apollo off-chain worker monitors on-chain price requests.
2. Once a price request for an asset has been detected, the off-chain worker does an http GET to the price-feed server and gets the latest price.
3. If and only if the latest cached price (on the price-feed server) is recent enough (in seconds, configurable via the price-feed CLI), the off-chain worker submits a transaction containing the asset price.


## Setting up a node

[Setup a node by following the collator guide](https://docs.composable.finance/developer-guides#collator-set-up-guide)


### Setting up a node in development mode

For development mode, polkalaunch scripts should be used. 

It sets up a local network with 4 collators with predefined keys and addresses.

**Setup required prerequisites**

A **Debian** based Linux system is required, we recommend Debian, Ubuntu or Linux Mint.

1. Set up required packages 

Run the following command:


```bash
sudo apt update && sudo apt install -y git clang curl libssl-dev llvm libudev-dev pkg-config wget
```

2. Setup Rust binary and Toolchain

Run the following commands:


```bash
RUST_C="nightly-2021-11-07"
curl https://sh.rustup.rs -sSf | sh -s -- -y && \
export PATH="$PATH:$HOME/.cargo/bin" && \
rustup toolchain uninstall $(rustup toolchain list) && \
rustup toolchain install $RUST_C && \
rustup target add wasm32-unknown-unknown --toolchain $RUST_C && \
rustup default $RUST_C && \
rustup show
```

And wait for the installation process to finish.

3. Setup Nodejs & Yarn 

Run the following commands:

```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.35.3/install.sh | bash && \
export NVM_DIR="$HOME/.nvm" && \
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh" && \
nvm install v16.15.0 && \
nvm use v16.15.0 && \
npm install --global yarn
```

**Setup Polkalaunch script**

Run the following commands:

Getting binaries

```bash
cd ~/ && git clone --depth 1 --branch v2.2.1 https://github.com/ComposableFi/composable.git && \
cd composable
```

```bash
wget -P ./target/release https://github.com/ComposableFi/composable/releases/download/v2.2.1/composable && \
chmod u+x ./target/release/composable
```

```bash
cd ~/
wget -P ./polkadot/target/release https://github.com/paritytech/polkadot/releases/download/v0.9.18/polkadot && \
chmod u+x ./polkadot/target/release/polkadot
```

Installing dependencies

```bash
# install project
cd ~/composable/scripts/polkadot-launch
yarn install 
```

To start your local composable chain

```bash
yarn composable
```

Once it’s done you would see something like this in the terminal


```markdown
ubuntu@oracle-test:~/composable/scripts/polkadot-launch$ yarn composable
yarn run v1.22.18
$ rm -rf /tmp/polkadot-launch && polkadot-launch composable.json

🧹 Resolving parachain id...
2022-05-23 10:11:09 Building chain spec    
  ✓ Read parachain id for ../../target/release/composable: 2000
2022-05-23 10:11:10 Building chain spec    

🧹 Starting with a fresh authority set...
  👤 Added Genesis Authority alice
  👤  Added Genesis Authority bob
  👤  Added Genesis Authority charlie
  👤  Added Genesis Authority dave
  👤  Added Genesis Authority ferdie
```

Followed by: 

```markdown
Starting a Collator for parachain 2000: 5Ec4AhPUwPeyTFyuhGuBbD224mY85LKLMSqSSo33JYWCazU4, Collator port : 31300 wsPort : 9996 rpcPort : undefined
Added --charlie
Added --force-authoring
Added --chain=dali-dev
Added  to parachain
Added --execution=wasm to collator
🚀 POLKADOT LAUNCH COMPLETE 🚀
```

This means your node has started.

Nodes are writing logs here: 

```markdown
ubuntu@oracle-test:~/composable/scripts/polkadot-launch$ ls
9988.log  9997.log   alice.log  charlie.log      composable_and_basilisk.json  ferdie.log      node_modules  rococo-local-raw.json  yarn.lock
9996.log  README.md  bob.log    composable.json  dave.log  

ubuntu@oracle-test:~/composable/scripts/polkadot-launch$ tail -f 9988.log 
2022-05-23 10:23:24 [Parachain] PoV size { header: 0.1787109375kb, extrinsics: 2.4931640625kb, storage_proof: 5.80078125kb }
2022-05-23 10:23:24 [Parachain] Compressed PoV size: 7.423828125kb
2022-05-23 10:23:24 [Parachain] Produced proof-of-validity candidate. block_hash=0x67087d9d563ecbe2f13ab63d4280f003f80a4189be3f800c12adc82361463a2d
2022-05-23 10:23:25 [Relaychain] 💤 Idle (7 peers), best: #113 (0xb71b…7e30), finalized #110 (0x0ffc…245c), ⬇ 5.7kiB/s ⬆ 6.0kiB/s    
2022-05-23 10:23:25 [Parachain] 💤 Idle (2 peers), best: #44 (0x88ff…32ad), finalized #42 (0xc49f…12c0), ⬇ 0.1kiB/s ⬆ 1.4kiB/s    
2022-05-23 10:23:30 [Relaychain] ✨ Imported #114 (0x496f…c871)    
2022-05-23 10:23:30 [Relaychain] ♻️  Reorg on #114,0x496f…c871 to #114,0x9970…5f18, common ancestor #113,0xb71b…7e30    
2022-05-23 10:23:30 [Relaychain] ✨ Imported #114 (0x9970…5f18)    
2022-05-23 10:23:30 [Relaychain] 💤 Idle (7 peers), best: #114 (0x9970…5f18), finalized #110 (0x0ffc…245c), ⬇ 4.5kiB/s ⬆ 4.0kiB/s    
2022-05-23 10:23:30 [Parachain] 💤 Idle (2 peers), best: #44 (0x88ff…32ad), finalized #42 (0xc49f…12c0), ⬇ 24 B/s ⬆ 24 B/s    
2022-05-23 10:23:34 [Relaychain] 👴  Applying authority set change scheduled at block #111   
```

## Using PolkadotJS Web Interface

To see the block explorer and run extrinsics the PolkadotJS web interface needs to be connected. 

* Go to polkadot js →[https://polkadot.js.org/apps/#/explorer](https://polkadot.js.org/apps/#/explorer)
* Add custom endpoint

Connection should be established to port 9988. ws://127.0.0.1:9988 \
Please note that port 9944 doesn't have pallet functionality; it is a relay node. 

Make sure you have connected to the right port. 

![polkadotjs_web_interface](../../static/img/developer-guides/oracle/polkadotjs-web-interface.png)

You should see the block explorer:

![block_explorer](../../static/img/developer-guides/oracle/block-explorer.png)

In this web UI we will run extrinsics & RPCs, to attach the price feed to the node.

## Setting up the price feed (reference implementation)

Composable provides a reference implementation for the price-feed server. It can be found 

at the following address, in the 
[Composable GitHub repository](https://github.com/ComposableFi/composable/tree/main/utils/price-feed). 
The implementation is general enough and allows any fork to implement a new feed easily. 
By default, the prices are fetched from the 
[Binance public websocket API](https://docs.binance.org/api-reference/dex-api/ws-streams.html#4-trades).

### Setup

In this step, we will set up a rust compiler, a toolchain and build a node.  \
 \
**Setup required libraries**

Run the following command:

```bash
sudo apt update && sudo apt install -y git clang curl libssl-dev llvm libudev-dev pkg-config
```

**Get the project and build the price-feed**

Run the following commands:

```bash
git clone --depth 1 --branch v2.2.1 https://github.com/ComposableFi/composable.git composable-oracle && \
cd composable-oracle && \
cargo build --release --package price-feed
```

### Start Price-feed

You can try running the server with  \

```bash
RUST_LOG=debug ./target/release/price-feed
```

The server will start indexing a list of predefined assets (hardcoded).

To make sure the price-feed is working correctly one should go to the browser.

By default, price-feed runs on localhost, port 3001. 

Currently only the price for KSM is supported, which can be accessed using the following link.

[http://127.0.0.1:3001/price/4](http://127.0.0.1:3001/price/4)


![price_feed_output](../../static/img/developer-guides/oracle/price-feed-output.png)


price-feed output should look like this.

```markdown
[2022-06-09T19:08:25Z DEBUG price_feed::backend] notification received: Started { feed: Binance }
[2022-06-09T19:08:25Z INFO  price_feed::backend] Binance started successfully
[2022-06-09T19:08:25Z DEBUG price_feed::backend] notification received: AssetOpened { feed: Binance, asset: KSM }
[2022-06-09T19:08:25Z INFO  price_feed::backend] Binance has opened a channel for KSM
[2022-06-09T19:08:25Z DEBUG price_feed::feed::binance] connecting to binance
[2022-06-09T19:08:25Z DEBUG tungstenite::client] Trying to contact wss://stream.binance.com:9443/stream?streams=ksmusdt@aggTrade at 13.114.11.14:9443...
[2022-06-09T19:08:26Z DEBUG tungstenite::handshake::client] Client handshake done.
[2022-06-09T19:08:26Z DEBUG price_feed::feed::binance] running event loop
[2022-06-09T19:08:31Z DEBUG price_feed::backend] notification received: AssetPriceUpdated { feed: Binance, asset: KSM, price: TimeStamped { value: (Price(6659), Exponent(2)), timestamp: TimeStamp(1654801711) } }
[2022-06-09T19:08:36Z DEBUG price_feed::backend] notification received: AssetPriceUpdated { feed: Binance, asset: KSM, price: TimeStamped { value: (Price(6659), Exponent(2)), timestamp: TimeStamp(1654801716) } }
```

The default URL is:

```markdown
localhost:3001/price/${ ASSET_ID }/
```

### CLI Options

Run the following command: 

```bash
./target/release/price-feed --help
```

For a list of CLI options.


```markdown
ubuntu@oracle-test:~/price_feed/target/release$ ./price-feed --help
price-feed 1.0
Composable

USAGE:
    price-feed [OPTIONS]

OPTIONS:
    -c, --cache-duration <CACHE_DURATION>
            Duration, in seconds, before a price is evicted from the cache [default: 10]

    -e, --expected-exponent <EXPECTED_EXPONENT>
            Price will be normalized to this exponent [default: 12]

    -h, --help
            Print help information

    -l, --listening-address <LISTENING_ADDRESS>
            Listening address for the frontend [default: 127.0.0.1:3001]

    -p, --pythd-host <PYTHD_HOST>
            Host address of the pythd server [default: http://127.0.0.1:8910]

    -q, --quote-asset <QUOTE_ASSET>
            Asset to be used as quote for pricing [default: USDT]

    -V, --version
            Print version information
```

## Setting up Apollo (becoming an Oracle)

In order for a Collator to become an Apollo Oracle, you need to make sure that you deployed a price-feed server along 
your node. 

Once you have node and price-feed setup and running, the following steps will be required to bind your price-feed to the 
node.

These are the wallet details for the Alice developer wallet.

It’s required for registering the offchain worker.

```markdown
name: "Alice"
Address (public key): "5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL"
Mnemonic(seed): "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice"
```

### 1. Automated Setup

This describes the automated setup using our oracle initialization script.

Please scroll down to part 2. For manual setup instructions.

**Setup required libraries**

Installing application dependencies:

```bash
sudo apt update && sudo apt install -y git curl
```

Installing NodeJS:

```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.35.3/install.sh | bash && \
export NVM_DIR="$HOME/.nvm" && \
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh" && \
nvm install v16.15.0 && \
nvm use v16.15.0 && \
npm install --global yarn
```

**Setup oracle price feed initializer**

Getting the oracle price feed initializer:

```bash
git clone --depth 1 https://github.com/ComposableFi/composable.git composable-oracle-initializer && \
cd composable-oracle-initializer/scripts/oracle-setup
```

Setup oracle price feed initializer:

```bash
yarn
```

Starting oracle price feed initializer:

```bash
yarn start
```

**Registering offchain worker**

As soon as the script has finished, only a single step remains.

We need to register the offchain worker on our chain.

To do this go to:

_Developer menu -> RPC calls -> author -> InsertKey_

And enter the details, as seen in the screenshot below and press: “Submit RPC call”.

```markdown
keyType: orac
suri: bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice
publicKey: 5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL
```

![register_offchain_worker](../../static/img/developer-guides/oracle/register-offchain-worker.png)


After successfully following these steps, you should see the blockchain successfully getting prices submitted, every few
blocks.


![successful_price_submitted](../../static/img/developer-guides/oracle/successful-price-submitted.png)


### 2. Manual Setup

For the manual setup we need to do the following:

1. Register price feed URL
2. Register offchain worker
3. Set signer
4. Create an oracle for the asset

**Register price feed URL**

Register your price-feed url in the local storage,` kind` must be **PERSISTENT**, the key is **ocw-url,** and the value 
is **[http://localhost:3001/price/](http://my-price-feed.com/prices/)**

JavaScript:

```JavaScript
api.rpc.offchain.localStorageSet("PERSISTENT", stringToHex("ocw-url"), stringToHex("http://localhost:3001/price/"));
```

To do this go to:

_Developer menu -> RPC calls -> offchain -> **localStorageSet**_ \


![manual_register_price_feed](../../static/img/developer-guides/oracle/manual-register-price-feed.png)


**Register offchain worker**

To do this go to:

_Developer menu -> RPC calls -> author -> **insertKey**_

And enter the details above, as seen in the screenshot and press: “Submit RPC call”.


![register_offchain_worker](../../static/img/developer-guides/oracle/register-offchain-worker.png)


**Setting signer**

Bond the controller account by submitting a set_signer transaction (tie the Signer to the Controller). This transaction 
**must** be sent by the controller. The controller **must have the necessary bond amount** as it will be transferred to 
the signer and put on hold (reserved).

JavaScript:

```JavaScript
api.tx.oracle.setSigner(address);
```

Setting the signer automatically adds a small amount of funds to the oracle stake of this wallet. These are required for
submitting prices.

_Developer -> extrinsics -> Oracle -> setSinger_


![manual_set_signer](../../static/img/developer-guides/oracle/manual-set-signer.png)


**Create oracle for asset**

We can create an oracle for an asset by using the extrinsic _oracle.addAssetAndInfo(...)_ and calling it using 
administrative rights. To call this extrinsic as an administrator go to:

_Developer -> Sudo -> Oracle **-> addAssetAndInfo	**_

**Parameters**:

* assetId: 4
* threshold: 80
* minAnswers: 1
* maxAnswers: 3
* blockInterval: 6
* reward: 10000
* slash: 10000

![create_oracle_for_asset](../../static/img/developer-guides/oracle/create-oracle-for-asset.png)


![authorize_transaction](../../static/img/developer-guides/oracle/authorize-transaction.png)


After successfully following these steps, you should see the blockchain successfully getting prices submitted, every few
blocks.


![successful_price_submitted](../../static/img/developer-guides/oracle/successful-price-submitted.png)


____

External Link 

[Polkadotjs API](https://polkadot.js.org/docs/)
