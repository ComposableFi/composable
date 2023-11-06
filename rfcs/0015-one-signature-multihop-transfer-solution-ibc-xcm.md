# Overview

Composable is first chain which transfers tokens from Dotsama XCM chains to Cosmos IBC and back.

It would be nice have a solution allowing to do multi chain transfers with one wallet signature and handling fails.

This proposal describes the solution to do such transfers.

It accepts current state of multichain transfers, and tries to expand on this, without heavy code forks and modifications, retaining some compatibility with existing indexers.
Proposal optimizes for liquidity and initial user experience using some well know approaches, rather than target to final ultimate solution, like ICS-999, Composable CVM and Parity XCM (none of these are multichain production ready).

## Prerequisites of understanding

- [IBC whitepaper solution architecture](https://arxiv.org/pdf/2006.15918.pdf) 
- [ICS-020 token transfer application](https://github.com/cosmos/ibc/tree/main/spec/app/ics-020-fungible-token-transfer)
- [ICS-004](https://github.com/cosmos/ibc/tree/main/spec/core/ics-004-channel-and-packet-semantics) send/receive/acknowledgement/timeout of packets. 
- IBC modules specification and sample implementations. 
- Parity XCM format and configuration.
- [packet-forward-middleware(PFM)](https://github.com/strangelove-ventures/packet-forward-middleware)
- Substrate Pallets

## Out of scope

What tokens to be send and what chains are priority and how handle setup on Cosmos chains.

## What next

Document describes execution of few heterogenous multihop scenarios for  consideration.
Other scenarios can be deduced.

## Flows

### Parity Polkadot(Substrate) -> Composable Composable(Substrate) -> Composable Picasso(Substrate)

**On Polkadot**

Currently Polkadot is very restricting in what XCM messages can be issued from it, hence we vary destination multilocation. 

Send `DOT` transfer to `parent = 0, parachain = Composable, pallet = PalletXcmIbc, account = Alice, index = Picasso, account = Bob`

Detailed description of above line is here:
```
parent = 0 - Polkadot has no consesus parents
parachain = Composable - parachain id number
pallet = PalletXcmIbc - number of pallet `pallet-xcm-ibc` in Composable runtime
account = Alice - sender account, , 32 bit account address, because XCM erases original sender signature
index = Picasso - number of final destination network id in `pallet-network-registry`
account = Bob - receiver account on final chain
```

We will continue to use strings in multilocation in document, so these are numbers in implementation. 
Usage of general key bytes in multilocation is considered bad. So avoiding usage until forced to.

Pallets will be descried later.

**On Composable**

`XCM` messages callbacks new pallets on Composable to route transfer to `Picasso` over IBC.

**On Picasso**

`Bob` gets `DOT` on `Picasso` 

### Parity Polkadot(Substrate) -> Composable Composable(Substrate) -> Composable Picasso(Substrate) -> Osmosis(Cosmos SDK)


**Polkadot**

`parent = 0, parachain = Composable, pallet = PalletXcmIbc, account = Alice, index = Osmosis, account = Bob`

**Composable**

Same as previous, but sender to `Picasso` uses `memo` according `PFM` to forward transfer to `Osmsosi`

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

`PFM` middleware parses memo sends transfer to `Osmosis`. 
Details of forwards are in `PFM`. Some details of Rust implementation are down in the document. 


### Centauri(Cosmos SDK) -> ->  Composable Picasso(Substrate) -> Composable Composable(Substrate) -> Bifrost(Substrate) 

**Centauri**

Sends transfer to `Picasso` with 

```json
{
  "forward": {
    "receiver": "composable-alice",
    "port": "transfer",
    "channel": "channel-123",
    "next": {
      "substrate" : {
          "parent" : "Polkadot",
          "parachain" : "Bifrost",
          "account" : "alice", 
      }
    }
  }
}
```

*Details*

Solution will work for Centauri were Composable controls over JSON. 

For [Osmosis](https://github.com/osmosis-labs/osmosis/blob/main/cosmwasm/packages/registry/src/registry.rs), `substrate` part will be ignored for parsing, and yet forwarded.

**Picasso**

Sends transfer to `Composable` with `memo`:
```json
{
    "substrate" : {
        "parent" : "Polkadot",
        "parachain" : "Bifrost",
        "account" : "alice", 
  }
}
```

**Composable**

Handles IBC memo and sends XCM transfers to Bifrost.

*Details*

On each step must check that transfer amount forwarded never more than amount received.
Take fees or not.

### Polkadot -> Composable -> Picasso -> Centauri -> Osmosis

Add one more Network and forwarding rule template to finally jump to Osmosis.

*Notes*

Generally so many hops are bad. 
Slow, assets can hang or become illiquid in one hop in case of IBC down on any hop, hard to trace, lowered security (asset as insecure as lowers security of any hop).

Better routes are
- `Polkadot -> Composable -> Centauri -> Osmosis`
- `Polkadot -> Centauri -> Osmosis`

## Limitations

Cannot send several assets or NFTs at same time. 

Cannot express(securely) arbitrary routing and programs execution.

Cosmos IBC people working on this, we can migrate with them later.

## Fees

Any hop governed by Composable may configure any fee taken or not, and what subsidies IBC relayers will get to route messages.

Fees are not part of this RFC.

## Error handling

Go module for port forwarding has good explanation of error handling and is reference source porting code to Rust.

See details of XCM error handling in detailed description of XCM part of protocol.

## XCM

XCM multi hop transfers [do not fail](https://substrate.stackexchange.com/questions/6831/how-does-the-CVM-architecture-ensure-the-absoluteness-principle-described). 
Their `ExportMsg` has relevant interface which would be used for ultimate integration with IBC, but we are not here yet.

So we hack here with 2 pallets, which we can migrate to more official ways later.

### pallet-network-registry

Contains mapping for `index` identifiers of networks to channel port routes to final chain.

The pallet is configured on Composable.

`1` would map to `transfer/channel-15` to reach `Picasso`.
`3` would map to `transfer/channel-15/transfers/channel-42` to reach `Centauri`.
`4` would map to `transfer/channel-15/transfers/channel-42/transfer/channel-123` to reach `Osmosis`.

This information is enough to route packet from Polkadot to proper IBC route via `PFM`.

Additional metadata about relevant default timeouts can be stored too.

Pallet if given with original multilocation, can provide XCM route to send tokens to `Alice` on original chain. 
So given `parent = 1, pallet = PalletXcmIbc, account = Alice, index = Osmosis, account = Bob`  it can standard XCM multilocations which can be send over `pallet-xtokens`. 

### pallet-xcm-ibc

Pallet handles callbacks for XCM configuration in runtime. And pallet has similar level of capabilities like IBC middleware module (receive memo and packet lifecycle hooks).

So pallet receives original multilocation route and tokens. It sends tokens via route using `PFM` using data from `pallet-network-registry`.

Pallet tracks original multilocation route to packet commitment key to be able to send tokens back via XCM in case of IBC failure.
Account derived from original multilocation would be sender of tokens.

### Sending and receiving

#### XCM sends tokens to IBC

`pallet-xcm-ibc(PXI)` uses `pallet-ibc` to send tokens via IBC, proper routed is built using `pallet-network-registry` data.

PXI gets callbacks for IBC ACKs and timeouts.

In case of success ACK, only removes tracked multi location.

In case of failure (timeout or fail ACK) callback from `pallet-ibc` , tokens are unescrowed to account mapped to original multilocation. 

Pallet calls into `pallet-network-registry` to map multilocation to send relevant message via `pallet-xtokens`.

Sends tokens back to Polkadot. That route does not fail (there are no well known way to restore funds except vote on funds restoration track).

Pallet uses `pallet-assets` to check minimal fee to send tokens to `Polkadot`. If fees are not enough it does not send. Because fees are so small, we just eat them and ACK IBC packet.

#### IBC sends tokens to XCM


`PXI` handles `memo` callback from `pallet-ibc`. It parses memo. If `memo` contains substrate description of multilocation, and account, it forms `pallet-xtokens` and sends token to `Polkadot`.

In case of fees are ok, but route is wrong, we fail IBC ACK packet. So `PXI` is middleware which handles memo.

## Notes

Also account encoding never discussed, but assumption that it will take some time to be handle well. `pallet-network-registry` may require to store Cosmos network prefix for accounts.

Assumption that Centauri `pallet-ibc` will not need (substantial) modification to use `memo` handler to route transfers. 

Some safe limited form of [swaps](https://github.com/osmosis-labs/osmosis/blob/main/cosmwasm/contracts/crosschain-swaps/src/msg.rs) also may be supported like that, and also approach can be migrated to terminate memo destination with CosmWasm call (enabler for CVM).