# MANTIS

## The Problem

There is no one set definition for this Intents in DeFi. In general, intents are understood to be users’ desires for a given transaction or other outcome. Intents include desired parameters (such as to swap X amount of A token for B token), but leave some room for flexibility (such as where this swap occurs) in the solution that solvers provide. Anoma’s Apriori does a great job of breaking down the history and various definitions of intents in this blog about their Intents Day event. This blog by Essential also provides another means of defining intents and solvers.

Composable believes that:

- Intents are inherently cross-domain
- Blockchains are inherently trust-no-one markets

Distinct blockchain ecosystems, though increasing the solution space for problems, do not actually communicate with each other. There is thus a huge amount of potential loss due to lack of synchronicity between these markets. There is no credibly verifiable way to settle orders between chains, and also settle them at the same time. 

So, how can the supply chain be optimised so that there is the remote possibility of facilitating this?

## The Solution: MANTIS

### Architecture

MANTIS is comprised of the following components:

**Solvers**

In the [CVM](./cvm.md), a solver takes in data about users’ transactions, comes up with a solution to fulfill these transactions, and is incentivised to do so.

**Cross-Domain Communication via the IBC**

MANTIS leverages IBC to facilitate cross-chain intent settlement. IBC already connects Polkadot and Kusama to Cosmos appchains via Composable's work, with expansion to Solana and Ethereum in active development. 

**Multi-Domain Auctions**

User intents are scored based on volume cleared, with solutions being screened for MEV and bundled into a block for each domain.

**Language for Execution**

When the best solution is found, it is turned into a CVM program, which specifies the execution route.

**Verifiable Settlement**

Settlement of transactions resolving user intents must be verifiable. We also believe that these transactions must be partial block aware; To improve cross-domain censorship-resistance and enforce searcher conditioning for cross-domain transactions, partial block auctions are a must.

