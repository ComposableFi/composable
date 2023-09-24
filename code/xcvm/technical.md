# Overview

This document documents internals of CVM on CW and EVM.


### Transfers

CVM on CW Cosmos to Cosmos uses ICS20 for value transfers. Each ICS20 channel must be upserted into CVM config.

CVM uses ICS-20 assets transfers on Cosmos chains. On Polkadot and Ethereum it uses escrow/mint, path dependant semantics compatible with ICS-20.

### Bridges

CVM does not have any hardcoded requirement for bridge to be trustless or trustful. 
### Data encoding

Encoding is always deterministic (using deterministic subset of protobuf for example). 

Encoding depends on native encoding of chain, ability serde shared encoding and chain performance vs price variation.

A choice is based made on gas optimizations, engineering standards, and security practices.

### Asset identifier

Each asset id is 128 bit number with 4 first bytes are network id, means that numbers never overlap.

So it will not be the case that on one chain 123213 means PEPA and on other chain 123213 means SHIB.

Prefix allows to find network to look at for asset info and each chain introduce new assets independently.


## Deployments

Can be considered as 3 layers,

1. Full deployment of contract with all capabilities. Can do anything.
2. Partial fulfillment of CVM spec using on chain features in case not deployment is possible (usually can multi hop transfer, swap). 
3. Edges which may start or terminate CVM programs, but do not execute anything except simple sings (like one hop final transfer). `Shortcuts` capable doing only limited subset of operation on top of existing cross chain protocol.

For each chain and protocol it makes pragmatics solution to use existing liquidity and execution primitives.