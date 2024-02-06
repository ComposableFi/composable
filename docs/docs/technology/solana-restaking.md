# Solana Restaking

:::tip
The first opportunity for restaking on Solana is live. Users have a chance to get in on the action early and earn boosted rewards through [MANTIS games](../technology/solana-restaking/mantis-games.md) - a team staking competition on [mantis.app](https://mantis.app/).
:::

## What is Restaking?
Restaking has been [described as a new primitive](https://consensys.io/blog/eigenlayer-a-restaking-primitive) in crypto economic security that enables the rehypothecation of a token on the consensus layer. Specifically, the process of staking involves a user staking an ecosystem’s native asset to that ecosystem’s validators. The user then receives a receipt token representing this stake. They then “restake” this receipt token with validators again. This mechanism enables users to multiply the crypto economic security (and the yield) of their initial tokens, as they are essentially able to stake the same assets twice, receiving yield and supporting PoS validation both times.

Restaking has been pioneered and popularized by [EigenLayer](https://www.eigenlayer.xyz/), which is a protocol for restaking ETH on Ethereum. In particular, users staking ETH are able to opt into EigenLayer’s smart contracts for restaking their ETH and thus extending the crypto economic security to additional applications within the ecosystem. EigenLayer thus addresses rising concerns of fragmented security on Ethereum, helping to bootstrap the security of various protocols/applications. [EigenLayer’s total value locked (TVL)](https://defillama.com/protocol/eigenlayer) at the time of writing is over $275 million, indicating that there is a clear demand for restaking.

Despite the benefits of restaking, this concept has largely not yet expanded beyond the Ethereum ecosystem. However, there is a huge potential for restaking on other chains. This is particularly true on Solana, where there is a massive amount of staking occurring, with many prominent staking protocols already offering liquid staking tokens (LSTs) and receipt tokens that can be used for various purposes while a user’s original assets remain staked. In fact, at the time of writing, [over 392 million SOL are staked](https://solanacompass.com/statistics/staking), representing a staking market capitalization of over $25 billion dollars. This is a staggering 92% of the total circulating supply of SOL. Therefore, there is an incredibly large market for restaking these assets that are already staked in Solana. Yet, there has been very little use for these receipt tokens - until now.

## Why a Restaking Vault?
For the strength and security of the network, it is important to have validators powering the [Guest blockchain](../technology/solana-restaking/technical-overview.md) bootstrapped when the connection goes live. As the chain is secured via Proof-of-Stake (PoS), the [restaking vault solution](../technology/solana-restaking/vaults.md) will provide validators with an initial supply of (re)stake. 

Moreover, the restaking vault allows users to beat the crowd and participate in restaking in the Solana ecosystem before everyone else - and get rewarded for doing so. 

## How Restaking Will Work

The Restaking layer on Solana serves as the validation layer of the [Guest Blockchain](../technology/solana-restaking/technical-overview.md). This network needs to be validated like any other chain using the proof-of-stake (PoS) model.

Specifically, on the guest blockchain, previously staked assets are restaked with validators to secure the network. The security model involves control by a supermajority of nodes/validators on the guest blockchain. It is the nodes’ responsibility to sign corresponding payloads of transfer transactions. To join, a validator must provide a bonded stake. Thus, this model is gated from independent actors joining. Validators in the guest blockchain will be rewarded with a portion of bridging gas/transaction fees.

:::info
The restaking vaults accept staking of both Solana’s native SOL token as well as restaking of various receipt tokens for SOL staking platforms. These tokens can be staked with validators of the guest blockchain powering the Solana IBC connection. From these contracts, assets will be delegated to validators of [the guest blockchain](https://research.composable.finance/t/crossing-the-cross-blockchain-interoperability-chasm/33) that supports the IBC Solana connection. Thus, restaking in this manner will support the guest blockchain along the premise of PoS, which enhances the security of this connection.
:::

In this mechanism, it is critical that we properly determine the value of these restaked tokens. To accomplish this, oracles will need to be utilized to query different token pricing. The oracles can provide price feeds on token pairs, eg. stETH / ETH and provide a reasonable estimate of the current value based on the swap price. 

Users will accumulate staking rewards proportionate to their staking amount and time. Thus, they can receive not only the yield on their original stake, but also the yield from restaking.

## Governance
The restaking layer will initially be governed by a multisigs until a decentralised governance system is established. This is composed of a 7-of-9 multisig at address JD4dNpiv9G24jmq8XQMuxbQPKN4rYV7kue2Hzi1kNT4Q. As we move toward the launch of SOL IBC, we will look to expand this multisig in the greater pursuit of further decentralization. The **Admin & Upgradability multisig** is responsible for the following:

- Whitelisting tokens
- Setting the staking cap
- Setting if the guest chain is initialised or not
- Upgrading the contract
  
Signers for the multisig are as follows:

- Miguel Matos — Board member, Composable Foundation. Professor at the Universidade de Lisboa & Researcher at INESC-ID.
- Dan Edlebeck — Advisor, Composable
- Blas Rodriguez — CTO, Composable
- Joe DeTommaso — Head of Strategy, Composable
- Jafar Azam — Product Owner, Composable
- Dhruv D Jain — Research Analyst, Composable
- SolBlaze — bSOL LSD
- Don Cryptonium — Community Member
- Polkachu — Validator Operator