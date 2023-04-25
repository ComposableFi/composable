

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
 