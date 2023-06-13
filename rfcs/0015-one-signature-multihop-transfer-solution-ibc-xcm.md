# Overview

Composable is first chain which transfers tokens from Parity XCM chains to Cosmos IBC and back.

User experience it would be nice have as solution which will allow to do multi chain transfers with one wallet signature.

This proposal describes the solution to do such transfers.

It accept current state of multichain executions and transfers, and tries to expand on this, without heavy code forks and modifications(and some compatibility with existing indexers).
Proposal optimizes for liquidity and initial user experience using some well know approaches, rather than target to final ultimate solution, like ICS-999, Composable XCVM and Parity XCM envisions (but none of these are permissionlessly multichain production ready).

## Prerequisites

You have read or clear about [IBC whitepaper solution architecture](https://arxiv.org/pdf/2006.15918.pdf) and [ICS-020 token transfer application](https://github.com/cosmos/ibc/tree/main/spec/app/ics-020-fungible-token-transfer) and [ICS-004](https://github.com/cosmos/ibc/tree/main/spec/core/ics-004-channel-and-packet-semantics) send/receive/acknowledgement/timeout of packets. Awareness about IBC modules specification and implementation is useful too. 

You have read general overview of Parity XCM and is up to date with MultiLocation format.

You are aware of how assets encoded in IBC and in XCM.

## What next

Document describes very exact execution of few heterogenous multi hop scenarios we should consider.
Other scenarios can be deduced.


## Parity Polkadot(Substrate) -> Composable Composable(Substrate) -> Composable Picasso(Substrate)

**On Polkadot**

Currently Polkadot is very restricting in what XCM messages can be issued from it, hence we vary destination multi location. 

Send `DOT` transfer to `parent = 0, parachain = Composable, pallet = IBC,  index = 15, account = Alice`

**On Composable**

`XCM` configuration callback on Composable routes transfer to this location as call to IBC Transfer module over `channel-15` to `Alice` account.


**On Picasso**

Alice gets dots on `Picasso` 

*Details*

`index` encodes IBC `channel id` indicating it should be forwarded over it. 

`account` is 32 bit account address. 

`pallet` and `parachain` are numbers, here we just use strings for readability. 


## Parity Polkadot(Substrate) -> Composable Composable(Substrate) -> Composable Picasso(Substrate) -> Osmosis(Cosmos SDK)


**Polkadot**

`parent = 0, parachain = Composable, pallet = IBC,  index = 15, pallet = NetworksRegistry,  index = Osmosis, account = Alice`

*Details*

`pallet = NetworksRegistry,  index = Centauri` are numbers in hardcoded pallet like thing in `Picasso` runtime with several hardcoded chain ids to IBC routes mappings to avoid data initialization and migration complexity on first iterations.

**Composable**

When message arrived to `Composable` it transfers assets to it forms IBC transfer as above, but adds [memo](
https://github.com/strangelove-ventures/packet-forward-middleware):

```json
{
  "forward": {
    "receiver": "osmo-alice",
    "port": "transfer",
    "channel": "channel-123",
  }
}
```

**Picasso**

`MemoHandler` middleware parses memo sends transfer to `Osmosis`.


## Centauri(Cosmos SDK) -> ->  Composable Picasso(Substrate) -> Composable Composable(Substrate) -> Bifrost(Substrate) 

**Osmosis**

Sends transfer to `Picasso` with 

```json
{
  "forward": {
    "receiver": "composable-alice",
    "port": "transfer",
    "channel": "channel-123",
    "next": {
      "forward": {
        "substrate" : {
            "parent" : "Polkadot",
            "parachain" : "Bifrost",
            "account" : "alice", 
        }
      }
    }
  }
}
```

*Details*

Solution will work for Banksy were Composable controls over JSON. 

For [Osmosis](https://github.com/osmosis-labs/osmosis/blob/main/cosmwasm/packages/registry/src/registry.rs), `substrate` part will be ignored for parsing, and yet forwarded.

**Picasso**

Sends transfer to `Composable` with `memo`:
```json
{
    "forward": {
      "substrate" : {
          "parent" : "Polkadot",
          "parachain" : "Bifrost",
          "account" : "alice", 
      }
    }
}
```

**Composable**

Handles IBC memo and sends XCM transfers to Bifrost.


*Details*

On each step must check that transfer amount forwarded never more than amount received.

## Polkadot -> Composable -> Picasso -> Banksy -> Osmosis

Add one more Network and forwarding rule template to finally jump to Osmosis.

## Limitations

Cannot send several assets or NFTs at same time. 

Cannot express(securely) arbitrary routing and programs execution.

## Fees

To be handled in next RFC or implementation.

## Error handling

Go module for port forwarding has good explanation of error handling and source porting code to Rust.

See details of XCM error handling in detailed description of XCM part of protocol.


## XCM

XCM multi hop transfers [do not fail](https://substrate.stackexchange.com/questions/6831/how-does-the-xcvm-architecture-ensure-the-absoluteness-principle-described). 
Their `ExportMsg` has relevant interface which would be used for ultimate integration with IBC, but we are not here yet.

So our hack will require several pallets.

### pallet-network-registry

Contains mapping for `index` identifiers of networks to channel port routes to final chain.
 
In Composable `1` would map to `transfer/channel-15` to reach Picasso.
In Composable `3` would map to `transfer/channel-15/transfers/channel-42` to reach Centauri.
In Composable `4` would map to `transfer/channel-15/transfers/channel-42/transfer/channel-123` to reach Osmosis.

This information is enough to rout packet from Polkadot to proper IBC route via `packet forwarding middleware`.

Additional metadata about relevant defaults timeouts can be stored too.

Pallet callbacks into `pallet-xcm-ibc` with original `multi location` account and transferred tokens.

Pallet if given with original multi location, can provide do XCM message to send tokens to `Alice` on original chain. 
So given `parent = 1, pallet = IBC,  index = 15, pallet = NetworksRegistry,  index = Osmosis, account = Alice` , it can create and send XCM message via `pallet-xtokens` to transfer funds back.

### pallet-xcm-ibc

So pallet receives original multi location and tokens and route. It sends tokens via route using `port-forwarding-middleware`.

#### XCM sends tokens to IBC

In case of success ACK, pallet does nothing.

In case of failure (timeout or fail ACK) callback from `pallet-ibc` , tokens are unescrowed to account mapped to original multi location. 

Pallet callbacks into `pallet-network-registry` to map multilocation to send relevant message via `pallet-xtokens`.

### IBC sends tokens to XCM

Pallet handles `memo` callback from `pallet-ibc`. It parses memo. If `memo` contains substrate description of multilocation, and account, it forms `pallet-xtokens` and sends token to `Polkadot`.

## Notes

Also account encoding never discussed, but assumption that they will take some time to be handled well.

Assumption that Centauri `pallet-ibc` will not need (substantial) modification to use `memo` handler to route transfers. 

In theory some safe limited form of [swaps](https://github.com/osmosis-labs/osmosis/blob/main/cosmwasm/contracts/crosschain-swaps/src/msg.rs) also may be supported like that.

We may omit retries and changed timeout implementation initially in Rust codebase to simplify things.

For XCM location forward can consider using light client hash staff to inside `network_id` to encode IBC networks. 
