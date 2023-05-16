# About

Fast agile events, data changes and metrics tracking for on chain for monitoring and alerting.

# How to Developer


1. Ensure `nix .#subxt-codegen-picasso` and `subxt-codegen-composable` for latest metadata
2. Run metrics collector via nix run `nix run .#grafana-observe`.
3. Run light warp clients 

subxt-codegen-picasso



# Architecture

`blockchain(consensus)` -> changesets (logs/events/storage). in substrate - storage changesets

>

`client` to subscribe to data changesets (run you own or pay, not free), in substrate node or smoldot light client, must be robust to data


> 

`parser` - manages latest block up to which changeset where observed and allows to observe changesets from any block, decode

>

`enricher` - enrich changests with metadata (block number, on chain code version, asset identifiers, or some offchain data about assets, hashes, correlation ids, transaction id,  etc..), denormalize

> 

`correlator` - take several changesets and produce one entity (example IBC or XCM transfer, voting on governance), e.g. final event about IBC/XCM packet will check all previos messages and produce new data event with

>

`aggregator` - sum/average/index (like all XCM amount transefed, all IBC escows amounts)

>

`index` - allows to access data or subscribe to final

> 

`actor`  - ui to allow swap, pager duty, monitoring dashboard 


# This solution proof of concept

ousrouce hard stuff to other (reliabitliy, indexing, visiulaitaiton, alert)

## client+enricher (smoldot+subxt for substrat3)

1. warp is not here on our node, not all nodes has warp, storae are brittle, 
2. IBC -> trustless -> need `light client`(follow consensus -> follow data -> API)

## aggregator/index/actor

grafana

## biz

escrow account on sender chain = minted on receiver QQ

`escrow account on sender chain - minted on receiver != 0.5% for total too long then alert`

`token tranfer initiated - timeout amoutn - failed amount on composable = token recieved on picasso` can written

# Other people done (Substrate AND Cosmos and Ethereum)



 nix run .#subwasm -- metadata ~/Desktop/composable_runtime_v10021.wasm --format scale > composable.scale
 

 - `escrow amount on chain source == minted on target chain` <- collected all assets/balances/tokens events and account events, all escrow accounts changed events.
- `ibc sent per user = ibc received per user` <- same as above, sum of difference of grouped by `to` on sender and receiver is zero
- `account change for specific account should never all behind amount grouped by asset id` <- monitor all all accounts changesetsgit a
-- 64MrSHbjnbPACh3C7oUGHBvoyjdgnYEB8BRtTw9kptjgFTwx
-- 5wcgm3bbotHBcSjpgN6uGV338XhxDGuB22BETxsbgaMwqhzT
- `if there is no client update for 5 minutes alert` <- ibc all events
- `total number of token received envts` <- ibc transfer events
- `total number of token received events - (timeout + failures)` <- ibc transfer and packet events
- `total number of transfers of assets grouped by asset as grouped by chain` <- ibc transfer of events one direction and other direction
- `amount of successfull packets receive on target chain is less than sequence id on sender cahan`



## Metics

```
substrate_storage_system_account_free_account_5Ct3fNtVv4yFGd3qrzx1qrvZZVrKNr8fhTKmUHFqGXe8vpPd_chain_picasso_asset_id_1_amount


label are low cardianliy
asset_id x chain x channel x account x event type = <amount OR duration or ... number metric value>

```