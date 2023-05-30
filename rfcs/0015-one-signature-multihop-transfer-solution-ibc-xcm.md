# Overview

Composable is first chain which transfers tokens from Parity XCM chains to Cosmos IBC and back.

User experience it would be nice have as solution which will allow to do multi chain transfers with one wallet signature.

This proposal describes the solution to do such transfers.

It accept current state of multichain executions and transfers, and tries to expand on this, without heavy code forks and modifications(and some compatibility with existing indexers).
Proposal optimizes for liquidity and initial user experience using some well know approaches, rather than target to final ultimate solution, like ICS-999, Composable XCVM and Parity XCM envisions (but none of these are permissionlessly multichain production ready).

## Prerequisites

You have read or clear about IBC whitepaper solution architecture and ICS-020 token transfer application and ICS-004 send/receive/acknowledgement/timeout of packets.

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

`pallet = NetworksRegistry,  index = Bansky` are numbers in hardcoded pallet like thing in `Picasso` runtime with several hardcoded chain ids to IBC routes mappings to avoid data initialization and migration complexity on first iterations.

**Composable**

When message arrived to `Composable` it transfers assets to it forms IBC transfer as above, but adds [memo]
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


## Bansky(Cosmos SDK) -> ->  Composable Picasso(Substrate) -> Composable Composable(Substrate) -> Bifrost(Substrate) 

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

For [Osmosis](https://github.com/osmosis-labs/osmosis/blob/main/cosmwasm/packages/registry/src/registry.rs) it likely will work assuming that `substrate` part will be ignored, and yet forwarded. In case cannot  handle like that, either encode forwarding into `receiver` (like Polkadot encodes forwarding into receiver).


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

## Notes

Also account encoding never discussed, but assumption that they will take some time to be handled well.

Assumption that Centauri `pallet-ibc` will not need (substantial) modification to use memo handler to route transfers. 

In theory some safe limited form of [swaps](https://github.com/osmosis-labs/osmosis/blob/main/cosmwasm/contracts/crosschain-swaps/src/msg.rs) also may be supported like that.

We may omit retries and changed timeout implementation initially in Rust codebase to simplify things.