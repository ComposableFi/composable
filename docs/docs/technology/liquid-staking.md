# Polkadot Liquid Staking

Polkadot, utilising a Nominated Proof-of-Stake (NPoS) consensus model, allows DOT holders to stake their tokens to secure the network by nominating tokens to validators in return for yield. To enhance the utility of staked DOT, Composable has created a new liquid staking token (LST), called **Liquid Staked DOT (LSDOT)** that enables the liquidity to of staked DOT to be utilised without compromising the network's security. 

:::info
Liquid staking is a mechanism that allows holders of staked assets to access liquidity while continuing to participate in staking or securing a blockchain network, in this case, Polkadot. Traditional staking involves the locking of tokens for a specified duration to support network consensus and earn staking rewards, making these assets illiquid. Liquid staking protocols address this challenge by issuing fungible or representative tokens (e.g., staked versions of the original tokens) in exchange for the locked assets. 

These liquid tokens can then be freely traded, lent, or employed within decentralized finance (DeFi) applications, allowing users to navigate market fluctuations, generate additional yields, and actively engage with their assets while maintaining their staking commitments.
:::

A primary objective of Composable's liquid staking protocol is to facilitate users in unlocking liquidity from their staked DOT tokens, offering a range of significant advantages. This enhanced liquidity empowers users to explore a broader array of DeFi platforms, seize more yield opportunities, and improve capital efficiency. In essence, it will optimize users' financial flexibility and access to DeFi opportunities while maintaining the integrity of the Polkadot network, fostering a more dynamic and robust DeFi experience for the DOT token.

In the near future, this can be used to secure Composable’s ecosystem in line with the proof-of-stake (PoS) consensus model. This will be accomplished by allowing the restaking of LSDOT. **The result is pooled security**. This solution can be expanded to be offered to apps on Cosmos and other networks as Composable’s IBC connections continues to add new networks (such as to Ethereum and Solana).

Initially, the first step will be to facilitate the restaking of LSDOT into Composable's restaking layer. This restaking layer is vital as it not only allows for securing Composable's network via PoS, but also facilitates partial block building and thus relieves concerns over block proposer censorship and agency issues. Once this model is implemented on Composable's restaking layer, it can be integrated as a middleware and apps on various blockchains. As a result, it will create a use case for LSDOT in Cosmos and other networks, while delivering pooled security to the Composable ecosystem and beyond. 

Read more about why Composable has introduced Liquid Staked DOT [here](./liquid-staking/why-lsd.md).
