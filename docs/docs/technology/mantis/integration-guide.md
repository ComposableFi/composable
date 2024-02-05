# Integration Guides

## Integration Guide for Apps/Wallets
Wallets and Banana Gun bots will be able to plug into MANTIS using an open source UI package.

The end-to-end flow for getting data for solvers, posting the solution, and executing solutions on chain is as follows:

1. **User Posts Problem**
- The user sends their transaction to our Cosmos CosmWasm contract via RPC.
- The order ends up in storage.
- It is the user's responsibility to consult an oracle to set limits he wants.
- The problem format is [here](https://github.com/ComposableFi/composable/blob/f65076f5fcf2f0903b3d21e62ba22d7ba91c0c9f/code/xcvm/cosmwasm/contracts/order/src/lib.rs#L65).
  - In JSON format, in https://www.npmjs.com/package/cvm-cw-types?activeTab=code, open `/cvm-cw-types/dist/cw-mantis-order/response_to_get_all_orders.json`.

2. **Solvers Collect Data Needed to Solve Problems**
- AMM amounts/fees, tokens denominations, and routes are needed.
  - [AMM Neutron](https://app.astroport.fi/api/trpc/pools.getAll?input=%7B%22json%22%3A%7B%22chainId%22%3A%22phoenix-1%22%7D%7D)
  - [AMM Osmosis](https://app.osmosis.zone/api/pools?page=1&limit=300&min_liquidity=500000)
  - [Routes](https://github.com/ComposableFi/composable/blob/main/code/cvm/cvm.json)
  - Fees and rate limits are added if needed

- These are collected off-chain and on-chain.
- Coding is in progress to satisfy what the solver algorithm wants.

3. **Solvers Run Optimization Algorithm**
- Solvers solve for:
  - Maximal volume via Coincidence of Wants (CoWs), where CoWs are matched up to the limit.
  - The remaining amount of the order is proposed to be settled via constant function market maker (CFMM) routes
- Python coding is in progress [here](https://github.com/BrunoMazorra/2-assets-matching).
- Rust port is [here](https://github.com/ComposableFi/composable/blob/main/mantis/solver/src/solver.rs).
4. **Solvers Post Solution On-Chain Into Contract**
- Python solver output is in progress.
- The final matrix output is to be converted into a call to a standard Cosmos CosmWasm RPC.
- The solution message to RPC format is in https://www.npmjs.com/package/cvm-cw-types?activeTab=code, in the `/cvm-cw-types/dist/cw-mantis-order/response_to_get_all_solutions.json` file.
5. **Contract Choses the Solution Clearing the Largest Volume**
- Currently, any solution of several is picked each block for testing.
- The final configuration will be 2 blocks of solutions. 
- The [contract](https://github.com/ComposableFi/composable/blob/f65076f5fcf2f0903b3d21e62ba22d7ba91c0c9f/code/xcvm/cosmwasm/contracts/order/src/lib.rs#L343) checks that the solution respects the user's limits.
6. **Contract Executes CoW in the Same Transaction**
- CoWs happen on the same chain.
- These are coded as simple CosmWasm transfers back and forth between accounts.
7. **Contract Sends a Message to Convert Route to CVM**
- Each solution has a simplified version of routes which is mapped 1:1 to a more detailed lower level CVM program.
  - Coding is in progress.
The solution route tree is [here](https://github.com/ComposableFi/composable/blob/f65076f5fcf2f0903b3d21e62ba22d7ba91c0c9f/code/xcvm/cosmwasm/contracts/order/src/lib.rs#L153).
An equivalent root of the tree in the CVM is [here](https://github.com/ComposableFi/composable/blob/ee480d0062b8cde89e5cfb848881d88bb56f2625/docs/docs/technology/cvm/specification.md?plain=1#L120).
8. **The CVM is Executed**
- This mostly involves converting CVM instructions to IBC packets.

Problems can be submitted from a wallet or application via RPC to MANTIS. [Here](https://github.com/ComposableFi/composable/blob/06b2b265a4fb0e866faaf76af4ab94ba580560dd/docs/docs/technology/mantis/mantis.ts#L4) is an example typescript problem for submission to MANTIS.
