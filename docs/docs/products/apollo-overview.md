# Apollo

## An Overview of Apollo

Apollo is a decentralized oracle that provides trustless, 
flexible and permissionless access to off-chain data for applications built on Picasso. 
Oracles provide pricing data, verify proof of payments, and authenticate real-world events. 
This is achieved through a network of nodes, 
each of which collect and validate data from various external sources such as APIs 
and then broadcast this data to the network.

The process of data collection and validation is crucial to ensure the integrity of the data 
and prevent malicious actors from feeding false or manipulated information to a smart contract. 
This is achieved by ensuring all oracle providers need to put down a stake of PICA 
and if their answers deviate by a threshold from the mean, they are slashed. 
Furthermore, another security mechanism we implemented for how the data should be combined and processed is 
by taking an average of the data from all the nodes. 
This allows for greater flexibility in the types of data 
that can be used and the complexity of the logic that can be executed.

As a pallet on Picasso, 
Apollo can be leveraged by other pallets to form DeFi primitives that build on top of its oracle functionality. 
For example, [Angular Finance], which functions as a cross-chain money market and lending platform, 
would make use of Apollo’s functionality for retrieving price feeds on their assets.

[Angular Finance]: (https://www.angular.finance/)

## Mitigating MEV
Importantly, Apollo is MEV-resistant. 
MEV, or Maximal Extractable Value, is a form of front-running transactions that occurs at the block creation level. 
Apollo is able to overcome this challenge through its design, which also provides other security benefits. 
Normally, one common type of MEV allows actors to reorganize transactions in a block.
For example, in a proof stake blockchain, 
a validator could see a new price update before it’s executed through the oracle 
which could allow them to include a transaction to buy or sell an asset based on the expected price (frontrunning).

With the use of substrate blockchain `on_initialize` hook in Apollo,
the order of the Oracle price updates is guaranteed;
i.e when a block initializes, the price of an asset is guaranteed a slot into the block. 
Apollo also implements an algorithm that instead of trusting one oracle for the price, 
it takes the median price from different oracles which significantly limits the ability of oracle manipulation.

![high-level-architecture](./apollo/high-level-architecture.png)

The implementation of how Apollo can be utilized is dependent on the developer. 
Apollo has asset ID types which represent an asset pair. 
Here's an example of how the architecture of Apollo submits the price of an asset:

- Firstly, a price is requested using the request price function (this can be built into pallets using Apollo)
- The request triggers an offchain worker to go out and query a local price feed. For more 
  information 
  on off-chain workers see [here} (https://substrate.dev/docs/en/knowledgebase/learn-substrate/off-chain-features)
- As the off-chain worker fetches the proper price from the oracle’s price feed, it prepares a transaction and 
  submits it back on-chain
- On the top of the next block if the threshold is met, 
  it will:
  - take the median price
  - check to make sure that no prices are too old 
  - either reward or slash the oracle providers
- Finally, the price and block number that was identified will be stored and available to the entire chain

Apollo is built to fulfill one task - provide an honest and non front-runnable price on-chain.

[Pallet Documentation](../pallets/oracle.md)

[Design Documentation](https://github.com/ComposableFi/composable/blob/main/frame/oracle/design/design.md)

