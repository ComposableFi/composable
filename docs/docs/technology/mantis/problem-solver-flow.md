An end to end flow description of getting data for solvers, posting the solution, and executing them on chain.


## Overview

These are steps which, detailed below, happen in this syste:
1. User post problems.
2. Solvers collect data needed to solve problems
3. Solvers run optimization algorithm
4. Solvers post solutions on chain into contract
5. Contract chooses solution with largest volume
6. Contract executes CoW in same transaction if there are COWs 
7. Contract sends message to convert route to CVM if there are no COWs 
8. CVM program executed

**Note: CVM information can be found here, essentially a DSL for intents: https://docs.composable.finance/technology/cvm 


### 1. User posts problem

User sends TX to Cosmos CosmWasm contract via RPC. 

Order ends up in storage.

It is the user's responsiblity to consult an oracle to set limits he wants.

**References**

Problem format is here:

https://github.com/ComposableFi/composable/blob/f65076f5fcf2f0903b3d21e62ba22d7ba91c0c9f/code/xcvm/cosmwasm/contracts/order/src/lib.rs#L65


In JSON format:

In https://www.npmjs.com/package/cvm-cw-types?activeTab=code open
`/cvm-cw-types/dist/cw-mantis-order/response_to_get_all_orders.json`


### 2. Solvers collect data needed to solve problems

AMM amounts/fees, tokens denoms, routes are needed.

These are collected offchain and onchain.

Coding in progress to satisfy what Solver algorithm wants.


**References**

AMM Neutron:
https://app.astroport.fi/api/trpc/pools.getAll?input=%7B%22json%22%3A%7B%22chainId%22%3A%22phoenix-1%22%7D%7D

AMM Osmosis:
https://app.osmosis.zone/api/pools?page=1&limit=300&min_liquidity=500000 

Routes:
https://github.com/ComposableFi/composable/blob/main/code/cvm/cvm.json

Fees and rate limits are added if needed.

### 3. Solvers run optimizaton algorithm

Solving for: 
1. Maximal volume via CoWs, CoWs matched up to limit
2. Remaining proposed to go via CFMM routes

Python coding is in progress.

**References**

Python
https://github.com/BrunoMazorra/2-assets-matching

Rust port:
https://github.com/ComposableFi/composable/blob/main/mantis/solver/src/solver.rs


#### Optimization

In general, when all routes take proportion fees from token operations (transfer/exchanges). 
This happens because solution may pick up any real value amid to points of doing operation with all input tokens or not doing at all.
Actually can find what part of tokens should go into

With cross transfers, there are constant fees regardless of transfer amount.
That makes need to decide to go to other chain or not.
That decision is binary and makes problem non convex (cannot take arbitrary point of like partially go to chain).
That means that some tricks and heuristics to be used convert mixed integer problem (integer whole values) to convex(real values),
and iterate picking route vs not route decisions semi randomly.


### 4. Solvers post solution on chain into contract

Python solver output in progress.

Final matrix output to be converted into call to standard Cosmos CosmWasm RPC. 

**References**

Solution message to RPC format is in
https://www.npmjs.com/package/cvm-cw-types?activeTab=code

in `/cvm-cw-types/dist/cw-mantis-order/response_to_get_all_solutions.json` file.


### 5. Contract chooses solution with largest volume from several.

Currently any solution of several is picked each block for testing.
Final config will be 2 blocks of solutions. 

Checks that solution respects user's limits.

**References**

https://github.com/ComposableFi/composable/blob/f65076f5fcf2f0903b3d21e62ba22d7ba91c0c9f/code/xcvm/cosmwasm/contracts/order/src/lib.rs#L343


### 6. Contract executes CoW in same transaction

CoWs happen on same chain.

Coded as a simple CosmWasm transfers back and forth accounts.

### 7. Contract sends message to convert route to CVM

Each solution has a simplified version of routes which is 1 to 1 mapped to a more detailed lower level CVM program. 

Coding is in progress. 

**References**

This is solution route tree 
https://github.com/ComposableFi/composable/blob/f65076f5fcf2f0903b3d21e62ba22d7ba91c0c9f/code/xcvm/cosmwasm/contracts/order/src/lib.rs#L153

Here is equivalent root of tree in CVM
https://github.com/ComposableFi/composable/blob/ee480d0062b8cde89e5cfb848881d88bb56f2625/docs/docs/technology/cvm/specification.md?plain=1#L120

### 8. CVM executed

That is mostly converting CVM instruction to IBC packets.