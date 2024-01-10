# Problem overview

We have [CVM language](https://docs.composable.finance/technology/cvm/specification) to describe cross chain execution and on chain routing registry over heterogenous networks.

**Programs**

CVM language is basically Abstract Syntax and Routing Tree which declares desire to move funds from chain to chain and execute some operations.


Simplified CVM:
```typescript
type Instruction = Deposit | Exchange | Spawn
interface Spawn {
    assets : Asset[]
    network_id : NetworkIdPgi
    instructions: Instruction[]
} 
type NetworkId = number
type AssetId = number
interface Asset {
    asset_id : AssetId
    amount : Amount
} 

type Amount = Percentage | Absolute
```

More complicated is [asset prefix elimination](https://api-docs.skip.money/docs/ibc-routing-algorithm), swap on some chain and moving assets to final destination.

**Registry**

Each chain and hop has its capabilities. One chain can have full set of transport features with contracts, other chain just can receive limited set of final assets and do not allow hosting programs.
Limited chains usually provide some mechanics to execute very specific set of scenarios provided by on chain modules.
Description of chains capabilities and assets they have is on chain registry.

On chain registry contains information on a lot on directly connected chains, and partial info on some 1+ hop connected, so as far chain is (and higher costs of on chain operation), 
the smaller registry is. Actually preventing global knowledge of all possible routes which can carry execution of CVM program.

Simplified registry:
```typescript
type TracePrefix = String
type NetworkFeatures = object
type ConnectionFeatures = object
interface Registry{
    asset_to_network(asset_id: AssetId) : NetworkId
    asset_id_after_transfer(asset_id: AssetId, to: NetworkId) : AssetId?
    asset_id_to_prefix(asset_id: AssetId): TracePrefix
    network(network_id: NetworkId): NetworkFeatures
    network_to_network(from: NetworkId, to: NetworkId): ConnectionFeatures?
}
```

**Next steps**

Next I will describe set of CVM programs, chain capabilities and registry information which should be solved in order express exact packets to send for execution.

After example I will summarize zoo of things into some categories and describe desired properties of algorithm and solutions to be found to handle user intents.

## Examples

Examples follows pattern:

* what expected user intent
* what is ideal execution
* what are CVM and registry subtleties

### Registry information

Please use table to refer to features available for chains and transport.

When `From/To` is same

| From/To    | Centauri  | Neutron                 | Osmosis                 | Stride                     | Cosmos Hub | Picasso | Composable | Polkadot              | Statemine             |
|------------|-----------|-------------------------|-------------------------|----------------------------|------------|---------|------------|-----------------------|-----------------------|
| Centauri   | Contracts | IBC                     | IBC                     | IBC                        | IBC        | IBC     |            |                       |                       |
| Neutron    | IBC       | Contracts, WH, Exchange | IBC                     | IBC                        | IBC        |         |            |                       |                       |
| Osmosis    | IBC       | IBC                     | Contracts, WH, Exchange | IBC                        | IBC        |         |            |                       |                       |
| Stride     | IBC       | IBC                     | IBC                     | ICA Callbacks, IBC Staking | IBC        |         |            |                       |                       |
| Cosmos Hub | IBC       | IBC                     | IBC                     | IBC                        | PFM        |         |            |                       |                       |
| Picasso    | IBC       |                         |                         |                            | IBC        | PFM     | IBC        |                       |                       |
| Composable |           |                         |                         |                            |            | IBC     | PFM        |                       |                       |
| Polkadot   |           |                         |                         |                            |            |         | XCM        | XCM Multihop Transfer | XCM                   |
| Statemine  |           |                         |                         |                            |            |         |            | XCM                   | XCM Multihop Transfer |

WH - Wasm Hook.
PFM - Packet Forwarding Middleware, can terminate with WH if final chain has it


### A

User wants `Transfer NTRN from Centauri to Osmosis and Exchange to OSMO`.

CVM programs describes:
1. Transfer NTRN to Neutron via IBC and call Contract
2. Neutron send NTRN to Osmosis
3. Call Contract to Exchange Contract to get OSMO

That is super happy path. We have Contracts and Wasm Hooks to do full CVM program.
CVM executed as is.

### B

User wants `Transfer ATOM from Centauri to Osmosis and Exchange to PICA`

What CVM prescribes:
1. Transfer ATOM to Cosmos Hub
2. Transfer ATOM to Osmosis.
3. Exchange to PICA

Cosmos Hub cannot execute Contracts.
So CVM should generate PFM packet which transfer 100% of assets to Cosmos Hub,
and Cosmos Hub sends it to Osmosis.
On Osmosis PFM ends with Wasm Hook call, where it actually end with CVM program or direct DEX call.

Algorithm should detect, using registry, that intermediate chain does not have Contracts,
and replace intermediate part with PFM forward.

PFM can transfer only 100% assets, so CVM program transferring part of assets from Cosmos Hub or doing some Exchange must be rejected.

### C

CVM program tells:

1. Transfer DOT from Osmosis to Centauri
2. From Centauri to Picasso
3. From Picasso to Composable
4. From Composable to Polkadot

So in this case full CVM program can transfer from Osmosis to Centauri.
But starting from Composable chain it is possible to transfer only 100% of assets, neither part nor absolute.

In this case 1-3 can be PFM transfer, and 3-4 can be XCM Multihop transfer.


### D

User intents to `Transfer PICA from Centauri to Polkadot`.

Whatever CVM program tells that, it must be rejected.

Polkadot can have only DOT.

### E

CVM program tells

1. Transfer PICA to Stride
2. Stake on Stride

Also CVM describes second operation as if it is happened on Stride, really ICA Callback must be formed by CVM program to send IBC packet to Stake.

Really CVM never reaches as some general AST tree to Stride.


### F

More general case.

Transfer from A to B to C and exchange on D.

A and D have Contracts. D has Wasm hook. B and C just PFM.

So routing algorithm for such CVM program must form sandwich like: `CVM(PFM(PFM(CVM)))`

### H

As above `F`, but D and C has Contracts.

Also D and C can do full CVM hops, why not just do cheaper PFM with direct WASM hooks of adapters?


### G

A chain knows what is on B chain, and B chain knows what is on C.
But chain A does not knows how B is connected to C

In case B has Contracts, we send CVM to decide.
In case B has no Contracts, but CVM is just transfer, we send PFM.
Because B to C will be single hop transfer.

Other cases rejected.


## Solution constraints and questions

Given CVM program(tree) and Registry data(graph) on chain, solution to provide annotated CVM program with hints on transports and payloads to use on each hop.

**On chain**

Solution to be executed on chain, cheap one, but still constrained in computation.

So it can be proven that on chain computations are impossible, and we need lower offchain.

**Limited information**

Each chain may have limited information about other chains, and yet we have to trust sender if he knows what he asks for.

So some CVM program to be rejected or some hope for best.
 
**Multiple spawns**

On chain A, program can split to move some assets to B and some to C.
So that both proper transport possibles to be found.

**Optimization(Optional)**

Price optimization, if can do Wasm Hooks/ICA Callbacks instead of full CVM when possible.
Do Transfer if CVM asks only for transfers, even if full contract based CVM possible.

**Hints(Optional)**

CVM program may have routing hints, telling what to do in case of limited information or price option.

Hints may be replaced by trustless CVM bots forcing Registry propagation.

## Possible ideas of implementation

CVM is tree.

Registry is graph.

```python
A

1 Start breadth first CVM traversing.

2 If connectivity options found, create traversal branch for each option.

3 Each child node gets path info which was done up to

  3.1 Check that given path info, can do operation outlined locally.

4 In good case, traversed path found

  4.1  Evaluate price and retain cheapest (CVM>PFM>IBC Transfer)

5 In case of not found path

  5.1 Use latest CVM step possible

  5.2 If no CVM before path not found, reject execution.

6 Output is sent as CVM output for generate detailed packets

6.1 CVM uses whole path metadata to generate proper transport payload

6.2 Including verify receiver on final chain (by checking hash of hash of path + original sender)
```


Also can vary like:

```python
B

1. If CVM is found next step - just do it

1.2 Assumed that next hop knows more about next hops 

2 If CVM is not immediate steps, but found on some substeps

2.1 Check that parent path allows CVM sending

2.2 Do 1

3. Do algorithm B
```


### Lowering

Ethereum too high price to execute above algorithm and Solana too constrained by heap/gas,
so any chain sending to Ethereum/Solana must lower CVM program into exact Ethereum/Solana ABI.