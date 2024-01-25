# Protocol Overview

Composable’s Multichain Agnostic Normalized Trust-minimized Intent Settlement (MANTIS) is an ecosystem-agnostic intent settlement framework. MANTIS facilitates settlement of cross-chain user intents, optimising the supply chain to deliver upon our vision of a user-centric, ecosystem-agnostic future for DeFi. 

Presently, MANTIS is live on mainnet for testing with a number of solvers to fulfill user intents, with the inclusion of oracles and collators to run validators for this framework.

:::info Understanding Intents

Intents have become a hot topic in DeFi, though there is no one set definition for this concept. In general, intents are understood to be users’ desires for a given transaction or other outcome. Intents include desired parameters (such as to swap X amount of A token for B token), but leave some room for flexibility (such as where this swap occurs) in the solution that solvers provide. Anoma does a great job of breaking down the history and various definitions of intents in [this blog](https://anoma.net/blog/intents-arent-real) about their Intents Day event. [This blog](https://blog.essential.builders/introducing-essential/) by Essential also provides another means of defining intents and solvers.
:::
## Intents are Inherently Cross-Domain

Composable's thesis introduces an additional feature to how intents are currently recognised, interoperability - intents are inherently cross-domain. To understand this, the history of the equities markets provides great insights as it became more efficient with execution being broken across different markets. Prior to the introduction of dark pools, which are private liquidity pools, individuals were only able to use select markets:

After the introduction of dark pools, which enabled orders to be broken up and split into different markets, the solution space broadened and more earnings opportunities were made available:

![dark-pool](../technology/mantis/overview.png)
## Blockchains are Inherently Trust-No-One Markets
Orders live on blocks that are publicly verifiable. A supply chain forms around these orders, involving the following entities and roles:

- **Users** - submit orders
- **Protocols** - serve as venues for orders
- **Block builders** - build blocks within orders
- **Proposers** - propose blocks with orders
- **Searchers** - extract MEV from the presence of these orders in pre-processed blocks

This relationship is shown below:

![flow](../technology/mantis/flow.png)

Distinct blockchain markets, though increasing the solution space for problems, do not actually communicate with each other. There is thus a huge amount of potential loss due to lack of synchronicity between these markets. Unlike the equities market, there is no credibly verifiable way to settle orders between chains, and also settle them at the same time.

How can the supply chain be optimized so that there is the remote possibility of facilitating this? **The solution is MANTIS.**