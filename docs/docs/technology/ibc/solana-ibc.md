# Solana IBC

Composable is working on implementing the Inter-Blockchain Communication (IBC) protocol on [Solana](https://solana.com/), establishing trust-minimized connections with Ethereum, Cosmos, and Polkadot. This accomplishment stands as a pioneering achievement, overcoming technical challenges that were previously deemed impossible.

Because of the requirements of implementing the IBC protocol, Solana and a number of other chains like [TRON](https://tron.network/) and [NEAR](https://near.org/) were previously thought to be incompatible with IBC. Composable has worked with collaborators at the University of Lisbon to develop a mechanism for making Solana and other IBC-incompatible chains capable of supporting IBC for the first time. Initially it will be deployed for Solana, with plans for expansion to other networks in the future.

The TL;DR of this innovation is that there will exist a guest blockchain inside of Solana deployed as a contract, providing all of the features needed to make Solana IBC-compatible. Validators on the guest blockchain receive messages about transactions on Solana, using this information to create blocks on the guest blockchain that reflect these Solana transactions. 

Thus, the guest blockchain essentially serves as a replication of the Solana blockchain, but unlike Solana, it is able to interoperate along the IBC and Composable’s trust-minimized bridge, trustless.zone. In this manner, the guest blockchain is a sort of layer 2 (L2) of the Solana network.

Through the Solana IBC connection, **restaking is enabled on Solana for the first time**. Restaking is a new and popular concept primarily established in the Ethereum community. In brief, restaking involves staking an asset with a blockchain’s validators along the standard proof-of-stake (PoS) mechanism, receiving some sort of receipt token for this, and then again staking the receipt token. Using the same amount of initial assets, this not only increases the yield a user can earn, but also enhances total security. These benefits are now being delivered into the Solana ecosystem as a necessary feature to implement Solana IBC. We are even further incentivizing staking on Solana via our restaking vault and team restaking competition as well.


## Benefits & Use Cases

Now, the massive usership and liquidity of Solana can flow into other IBC-enabled chains over Picasso and vice versa. Solana is a thriving ecosystem in terms of usership as well as in terms of value. As of December 7, 2023, the [total value locked (TVL) in Solana](https://defillama.com/chain/Solana) DeFi protocols is $712 million. This means Solana is the 7th largest blockchain in terms of TVL. The market cap of the network’s native SOL token was approximately $27.3 billion as of the same day. This puts SOL as the 6th largest token in terms of total market capitalization.

Moreover, this connection allows for the development of new cross-chain use cases between Solana and other ecosystems, and enables users to more seamlessly navigate the cross-chain DeFi space. With the Solana IBC connection, users and developers will be able to implement any kind of cross-chain use case imaginable between Solana and the other IBC-enabled chains, including Cosmos, Polkadot, Kusama, and Ethereum. Thus, the speed, cost-effectiveness, and massive liquidity and usership of Solana can be leveraged from other ecosystems as well.
