# Solana IBC

An implementation of IBC on [Solana](https://solana.com/) is live, establishing trust-minimized connections with Ethereum, Cosmos, and Polkadot. This accomplishment stands as a pioneering achievement, overcoming technical challenges that were previously deemed impossible. 

:::tip
Head to the [Picasso App](https://app.picasso.network/) and try it now!
:::

Due to the requirements of implementing the IBC protocol, Solana and a number of other chains like [TRON](https://tron.network/) and [NEAR](https://near.org/) were previously thought to be incompatible with IBC. In collaboration with the University of Lisbon, a solution has been developed for making Solana and other IBC-incompatible chains capable of supporting IBC for the first time. Initially it will be deployed on Solana, with plans for expansion to other networks in the future.

The TL;DR of this innovation is an AVS powered by the Solana Restaking Layer and deployed on Solana as a smart contract, providing all of the features needed to make Solana IBC-compatible. **Operators of the AVS receive messages about transactions on Solana, using this information to create blocks on the AVS that reflect these Solana transactions.** 

The [AVS for Solana IBC](../restaking/sol-ibc-avs.md) (previously referred to as the Guest Blockchain) serves as a replication of Solana, but unlike Solana, it is able to interoperate along the IBC landscape via [Mantis.app](https://games.mantis.app/). In this manner, the AVS for Solana IBC can be considered as a sort of Layer 2 (L2) of the Solana network.

:::tip Solana Restaking
Through the Solana IBC connection, **[restaking is enabled on Solana](../restaking.md) for the first time**. Restaking is a new and popular concept primarily established in the Ethereum community. In brief, restaking involves staking an asset with a blockchain’s validators along the PoS mechanism using liquid staked and receipt tokens belonging to the underlying L1.

This not only increases the yield a DeFi user can earn, but also enhances total security. These benefits are now being delivered into the Solana ecosystem as a necessary feature to implement IBC on the network. The restaking layer will be incentivized via the team staking competition designed for the [restaking vaults](../restaking/vaults.md).
:::

## IBC Requirements & The Need for an AVS for Solana IBC
IBC is an end-to-end stateful protocol for reliable, ordered, and authenticated communication between two blockchains. It enables bi-directional asynchronous communication between two blockchains within a relatively short time window (an average of less than one minute per IBC message ([Kim, Essaid, and Ju, 2022](https://ieeexplore.ieee.org/document/9919970/)). Thus, IBC is the only current mechanism of choice for facilitating cross-chain communication in a trust-minimized manner.

Yet, connecting to the IBC has a number of requirements. The IBC implementation on each blockchain has the following elements:

- **Provable key-value store**: this provides a key-value store interface with the addition that it can cryptographically prove to an external verifier the presence or absence of a given key-value pair. It is often realised as a Merkle trie but other cryptographic commitment schemes are also possible.

- **Counterparty’s light client**: an on-chain component responsible for processing and validating the blockchain headers of the counterparty blockchain. This client is light in the sense that processing only the headers requires a small amount of computational resources.

- **IBC module**: handles the logic of the IBC Protocol and maintains all the state necessary for the packet exchange. Some blockchains such as those implementing the CosmWasm specification have native IBC support and hence this module is part of the runtime environment. Other blockchains are IBC-agnostic and hence the IBC module is implemented as a regular Smart Contract.

- **Smart Contracts**: execute in the chain’s runtime environment and are responsible for sending and receiving IBC packets.

Additional technical requirements imposed by the IBC on chains that it connects are that the ledger needs to: 

- Provide a Smart Contract runtime with transactional state changes
- Support light clients and state proofs
- Provide block timestamps
- Support introspection including a view of past block hashes

Yet, not all chains meet these requirements. Notably, Solana does not offer state proofs, and instead uses a [simpler mechanism for payment and state verification](https://docs.solana.com/proposals/simple-payment-and-state-verification). The AVS for Solana IBC serves as a solution to this problem.

This [section](../restaking/sol-ibc-avs.md) outlines the approach taken for satisfying IBC requirements without having to extend the ledger implementation.  This solution can run on any blockchain which offers a Smart Contracts runtime.  We demonstrate it running on the Solana network and overcoming Solana’s lack of state proofs.

Audits for the IBC stack on Solana can be found on our [repository](https://github.com/ComposableFi/composable/tree/main/audits/solana-ibc-avs).

The IBC contract on Solana is currently managed by a 6/10 multisig, comprising both internal team members and external participants, including:

Don Cryptonium — Community Member
Polkachu — Validator Operator
Dan Edlebeck — Advisor, Composable
Rex St. John — Anza
Max Kaplan — Edgevana

This arrangement is temporary, pending the activation of cross-chain governance, after which the contracts will be governed by PICA stakers. The primary purpose of the multisig is to ensure contract upgradability.
## Assets
Assets entering Solana must also undergo whitelisting due to the constraints of Solana's data types, specifically, the u64 limit, which imposes restrictions on minting large amounts of tokens with large exponents.

Solana's u64 data type imposes limitations on the minting of tokens with large exponents, affecting assets bridged into Solana. Most Cosmos tokens, with 6 decimals, are not affected by Solana's constraints. 

To address this challenge, we have implemented an asset whitelisting contract that ensures compatibility between assets bridged into Solana and Solana's data type constraints. Through whitelisting, tokens undergo necessary adjustments, including decimal reduction for tokens exceeding 9 decimals, to guarantee seamless integration within the Solana ecosystem. Initially, there will be a 2/4 multisig that has the authority to whitelist accepted assets into the bridge. At a later time, PICA governance on Picasso will manage this process. It is important to note this multisig is not the owner of the IBC assets that are minted on Solana.

## Benefits & Use Cases

As a result of Solana IBC, the massive usership and liquidity of the ecosystem can flow into other IBC-enabled chains over Picasso and vice versa. Solana is a thriving ecosystem in terms of usership as well as in terms of value. As of December 7, 2023, the [total value locked (TVL) in Solana](https://defillama.com/chain/Solana) DeFi protocols is $712 million. This means Solana is the 7th largest blockchain in terms of TVL. The market cap of the network’s native SOL token was approximately $27.3 billion as of the same day. This puts SOL as the 6th largest token in terms of total market capitalization.

Moreover, this connection allows for the development of new cross-chain use cases between Solana and other ecosystems, and enables users to more seamlessly navigate the cross-chain DeFi space. With the Solana IBC connection, users and developers will be able to implement any kind of cross-chain use case imaginable between Solana and the other IBC-enabled chains, including Cosmos, Polkadot, Kusama, and Ethereum. Thus, the speed, cost-effectiveness, and massive liquidity and usership of Solana can be leveraged from other ecosystems as well.
