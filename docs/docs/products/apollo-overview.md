# Apollo

## An Overview of Apollo

Apollo is a primary pallet built on the Picasso parachain that functions as the ecosystem’s native oracle. 
An oracle provides up-to-date information on assets and transactions that occur outside of an ecosystem. 
Oracles provide pricing data, verify proof of payments, and authenticate real-world events. 
They are critical to the function of any smart contract that needs to interact with real-world data, 
and form a core part of any DeFi ecosystem.

As a primary pallet, Apollo is composable and modularly functional. As such, 
it can be leveraged by other pallets to form secondary and tertiary DeFi primitives that build on top of its oracle 
functionality. For example, a secondary pallet like [Angular Finance](https://www.angular.finance/), 
which functions as a cross-chain money market and lending platform, would make use of the Apollo primary pallet. 
Apollo is part of a wider array of primary pallet primitives that can be ‘stacked’ and combined to easily form new 
primitives.

Importantly, Apollo is MEV-resistant. MEV, or Miner Extracted Value, 
is a form of front-running transactions that occurs at the block creation level. 
Apollo is able to overcome this challenge through its design, which also provides other security benefits. Normally, 
one common type of MEV allows actors to reorganize transactions in a block. For example in a proof stake blockchain, 
a validator could see a new price update before it’s executed through the oracle 
which could allow them to include a transaction to buy or sell an asset based on the expected price (frontrunning). 
With the use of substrate blockchain on_initialize hook in Apollo, the order of the Oracle price updates is guaranteed, 
i.e when a block initializes, the price of an asset is guaranteed a slot into the block. 
Apollo also implements an algorithm that instead of trusting one oracle for the price, 
it takes the median price from different oracles which significantly limits the ability of oracle manipulation.

[Pallet Documentation](../pallets/oracle.md)

[Design Documentation](https://github.com/ComposableFi/composable/blob/main/frame/oracle/design/design.md)
