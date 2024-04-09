# Solana AVS Testnet Guide

:::info
This document serves as a guide for onboarding as an operatior of the AVS for Solana IBC AVS powered by the Picasso Restaking layer. Operators of this AVS are essentially validators of the previously known 'Guest Blockchain'. Additional information can be found [here](../technology/restaking/sol-ibc-avs.md).
:::

### Operator Security Model
The operator set of the AVS will be directed by majority where it is the responsibility of active validators to maintain uptime and sign corresponding payloads of transactions.

### Bonding

Joining as a validator will require a bonded stake to keep participation gated from malicious actors easily onboarding. The size of the bond will be 25 SOL.

### (Re) Staking
The validator set will be able to utilize liquid staked derivatives of SOL, such as jitoSOL, mSOL, bSOL, LST and edgevanaSOL.   

### Oracles
The Pyth oracle will be used to access price feeds for LST assets staked to the platform. 

### Slashing 
Slashing functionality will not be included during the initial launch stage. It will be implemented after the network is fully operational and IBC is live in production on Solana.

## Validator Setup

1. Install the validator CLI using the following command (From `validator-testing` branch) 
```
cargo install --git https://github.com/composableFi/emulated-light-client#validator-testing
```
2. Check if the validator CLI is installed using the following command. The current version should be returned as a value indicating successful installation.
```
validator --version
> 0.0.1
```
3. Set up the rpc url with validator keypair using the command below (note that the program ID is already added). Try to use custom 
rpc since the solana public rpc is not good enough to send transactions and will usually be dropped frequently. You can get the rpc
from helius, quicknode or triton. Keypair path is the path to your keypair json file. [For Example](https://github.com/ComposableFi/emulated-light-client/blob/2313bbd4c1f838ce36b894e781ede5eb63b7c698/solana/solana-ibc/keypair.json)
```
validator init --rpc-url <RPC_URL> --ws-url <WS_URL> --program-id 7uvnkZxh7Z1wwVFMQ1ak7u4LXWx9f8tkgUnMMyiZrSZb --keypair-path <KEYPAIR_PATH>
```
4. Once the config file is set, run the validator. 
```
validator run
```
:::note
You can even pass any of the arguments which would override the default config set in previous step. These arguments are
optional and has higher preference than the default config file. Any of the arguments can be passes and its not neccessary to pass
all of them.
```
validator run --rpc-url <RPC_URL> --ws-url <WS_URL> --program-id 7uvnkZxh7Z1wwVFMQ1ak7u4LXWx9f8tkgUnMMyiZrSZb --keypair-path <KEYPAIR_PATH>
:::
