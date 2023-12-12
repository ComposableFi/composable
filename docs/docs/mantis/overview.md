# Protocol Overview

Composable’s Multichain Agnostic Normalized Trust-minimised Intent Settlement (MANTIS) is an ecosystem-agnostic intent settlement framework. MANTIS facilitates settlement of cross-chain user intents, optimising the supply chain to deliver upon our vision of a user-centric, ecosystem-agnostic future for DeFi. 

Presently, MANTIS is live on mainnet for testing with a number of solvers to fulfill user intents, with the inclusion of oracles and collators to run validators for this framework.

## Understanding Intents

Intents have become a hot topic in DeFi, though there is no one set definition for this concept. In general, intents are understood to be users’ desires for a given transaction or other outcome. Intents include desired parameters (such as to swap X amount of A token for B token), but leave some room for flexibility (such as where this swap occurs) in the solution that solvers provide. Anoma does a great job of breaking down the history and various definitions of intents in [this blog](https://anoma.net/blog/intents-arent-real) about their Intents Day event. [This blog](https://blog.essential.builders/introducing-essential/) by Essential also provides another means of defining intents and solvers.

## Intents are Inherently Cross-Domain

Composable's thesis introduces an additional feature to how intents are currently recognised, interoperability - intents are inherently cross-domain. To understand this, the history of the equities markets provides great insights as it became more efficient with execution being broken across different markets. Prior to the introduction of dark pools, which are private liquidity pools, individuals were only able to use select markets:

After the introduction of dark pools, which enabled orders to be broken up and split into different markets, the solution space broadened and more earnings opportunities were made available:

(insert image)
## Blockchains are Inherently Trust-No-One Markets
Orders live on blocks that are publicly verifiable. A supply chain forms around these orders, involving the following entities and roles:

- Users - submit orders
- Protocols - serve as venues for orders
- Block builders - build blocks within orders
- Proposers - propose blocks with orders
- Searchers - extract MEV from the presence of these orders in pre-processed blocks

These distinct markets, though increasing the solution space for problems, do not actually communicate with each other. There is thus a huge amount of potential loss due to lack of synchronicity between these markets. Unlike the equities market, there is no credibly verifiable way to settle orders between chains, and also settle them at the same time.

So, how can we optimize the supply chain so that there is the remote possibility of facilitating this? The solution is MANTIS.

## Cross-Chain Intent Settlement: Benefits & Use Cases

### Improved Cross-Chain User Experience

At Composable, we believe that a user intent settlement platform (particularly, a cross-chain one) can improve the landscape for blockchain transaction execution. That is because this vastly improves the user experience, carrying out cross-domain exchange and abstracting away the complexity involved in this process. Furthermore, users do not have to spend time identifying the best opportunities to satisfy their intents, only to find that these opportunities are no longer available by the time that they have explored all options; instead this is done for them, in short order. 

### Cross-Domain MEV

A cross-domain intent settlement platform (such as that being developed by Composable) introduces a new type of MEV: cross-domain MEV. As this is a novel form of MEV and has mostly been considered a meme, the truth is, there has not yet been any applications which have introduced this conncept. Also, MEV is still a poorly studied and reported phenomenon, a number of questions thus arise. In particular, we at Composable believe that cross-domain MEV could impact the price of intent settlement. 

Cross-domain MEV can be defined as the extraction of value from cross-chain transactions. This extractible value originates from two primary sources ([McMenamin, 2023](https://arxiv.org/pdf/2308.04159.pdf)):

1. Intrinsic-extractable value: expected value for an extractor at the precise time the state or transaction must be acted on (t = 0). 
a. In an order, this is approximately the expected value of all front- and back-running opportunities combined.
b. In a pool, this is approximately the expected value from moving a price up or down at the time when orders are included in the chain.
2. Time-extractable value: derived similarly to an option, this is the value derived because the extractor has the time between confirmation times/blocks to determine if they should act on a particular blockchain state.

For extractors, this is the sum of all paths with a positive extractable value at expiration, multiplied by the probability of that path happening.

In the Composable ecosystem specifically, cross-domain MEV is potentiated from cross-chain intent settlement. Composable’s MANTIS receives user transaction intents, which are then picked up by solvers who compete to find the best solution to execute these intents. Once the optimal solution is chosen via a scoring mechanism, the winning solver must then execute upon their proposed solution. A single solution can involve a number of different domains. Searchers can access the orderflow from these solutions not only within each domain but also between domains, thus resulting in cross-domain MEV.

### Free/Reduced Gas

Another exciting potential benefit of a cross-domain intent settlement framework is reducing gas costs for users. This is detailed in our forum post here, but in brief, the way that gas costs could be kept as low as possible would be to have such gas costs be a dynamic value that is subject to market conditions. This means that users could be able to trade for free, but only in the event that the below incentive equation is positive, and solvers are able to cover user gas fees:

- (+) 0.1% of transfer, like CoW Swap 
- (+) Sale to blockbuilders
- (+) MEV
- (-) Money paid to blockbuilders in the role of searcher
- (-) User’s gas

If not, then users will have a partial gas payment. Solvers can also take out short term loans and use these to cover gas fees, then pay these loans off after the order is executed and they receive their rewards.

### Future Explorations

There are a number of ways in which Composable envisions expanding and improving our MANTIS intent settlement framework, once deployed. These include:

- Incorporating credible commitment schemes such as [MEV-Boost++](https://research.eigenlayer.xyz/t/mev-boost-liveness-first-relay-design/15?ref=blog.anoma.net) and [PEPC-Boost](https://efdn.notion.site/PEPC-FAQ-0787ba2f77e14efba771ff2d903d67e4?ref=blog.anoma.net) 
- Building a new relay that would allow for partial block building
- Moving towards a no-builder future where searchers can build blocks collaboratively and send them directly to proposers
- Implementing mempool matching and pre-reserved blockspace