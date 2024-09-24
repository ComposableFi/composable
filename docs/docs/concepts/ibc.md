# What is the IBC Protocol?

Before delving into an understanding of the IBC protocol, it's crucial to comprehend why choosing IBC is important and why the existing bridging solutions won't suffice to bring Decentralized Finance (DeFi) to the masses.

## Issues in cross-chain infrastructure 

The main objective of bridging infrastructure is to integrate the different blockchain asset and data markets to unify the fragmentation of liquidity. This integration is similar to how stock exchanges are connected through electronic trading to enable the transfer of capital. However, cross-chain bridges are currently facing significant safety issues due to the reliance on various trust assumptions that centralized bridges carry. 

Since 2021, exploits in DeFi have led to over 2 billion dollars in loss, with the most significant attacks occurring across multiple chains, where trusted bridges and CEX hot wallets accounted for over 52% of the total stolen funds in crypto. A trusted bridge is a connection between two blockchain networks that relies on a third party to facilitate the transfer of assets between the two networks. This third party is responsible for ensuring the security and reliability of the bridge, and users must trust this party to handle their transactions properly.

**If security is not prioritized, these risks will become even more widespread.**

During the market cycle of 2020-2021, as new ecosystems gained prominence, numerous bridging protocols emerged to address the growing need for liquidity. Most, if not all, of these bridges were built on optimistic, fraud-sensitive architectures, relying on trusted third parties, oracles, and multi-signatures. In addition to facilitating asset transfers, some of these bridges also supported message passing, which could serve as a foundational element for cross-chain applications. However, these bridges have proven to be security risks in DeFi and offer limited features to protocol builders.

In a trusted bridging setup, we identify the following actors:

- Relayer: pays for execution on the destination chain.
- Oracle: provides authorized mint and burn access to the contracts on the origin and destination chain.
- User: a contract, protocol or actual user directly interacting with the bridge. 

In this generic architecture, we choose to keep the relayer and oracle as separate actors, although in many actual implementations, these are the same entity. 

Designs used in Wormhole, Axelar and centralized exchanges use one or more accounts to determine the state of the origin chain, destination chain, and based on that, co-sign a message to mint or unlock funds. 

### Pessimistic & Optimistic bridges
**Pessimistic bridges** require the oracle to pre-validate withdrawals, assuming that relayers will commit fraud at its earliest opportunity.

The oracle claims to ensure the following responsibilities:

1. The event data is correct.
2. The event data is final.

For many chains, including Ethereum, the second responsibility cannot yet be ensured, and thus the oracle service is taking on a large risk. Usually this is reduced by waiting for a number of confirmations (blocks built on top of the current block).

From the on-chain perspective, funds are managed assuming that the oracle is honest about both responsibilities. Fraud, hacks or misconfigurations can lead to the oracle's authority being used to incorrectly release funds, as occured in Wormhole.

Different protocols attempt to reduce this risk by sharding the private key using multi party computation (MPC), or simply using a multisig.

For a secure bridging operation, the transaction time $t$ is given by:

$$ t := t_{finality} + t_{submission} $$

where $t_{finality}$ is the average duration for block finality, and $t_{submission}$ the length of time of block inclusion on the destination side.

**Optimistic bridges** assume that the relayer is usually honest, and fraud rarely occurs. The relayer/oracle algorithm is relatively identical to the algorithm. On the contract side however, the mint/unlock action is delayed, ensuring that fishermen have sufficient time to dispute the message.

Message acceptance is incredibly simple, for example:

```
\begin{algorithm}[H]
\SetAlgoLined
\BlankLine
\If{message is from relayer}{
    store(message)
}
\caption{Message acceptance protocol for optimistic trusted bridges}
\end{algorithm} 
```

However, actually enacting the message, which leads to mints and unlocks, there is a time delay, referred to as the dispute window.
```
\begin{algorithm}[H]
\SetAlgoLined
\BlankLine
\If{message received time is above wait threshold}{
  If{message is undisputed} {
    enact(message)
  }
}
\caption{Unlock protocol for optimistic trusted bridges}
\end{algorithm} 
```
Optimistic bridging trades in some advantages over pessimistic bridging, such as faster transaction times, in favour for more decentralized security.

For a secure bridging operation, the transaction time $t$ is given by:

$$ t := t_{finality} + t_{submission} + t_{dispute window} $$

where $t_{finality}$ is the average duration for block finality, $t_{submission}$ the length of time of block inclusion on the destination side, and $t_{dispute window}$ is the amount of time that the relayed transaction may be disputed on the destination side.

Relayers can choose to combine $t_{finality}$ and $t_ {dispute window}$, at the risk of needing to dispute their own submissions. This improves UX however, introduces a significant risk to the relayer, and in practice is not performed.

### Economic Risks in Trusted Bridging

Overall, bridging introduces security risks, both technically and economically. A wrapped token essentially represents a debt obligation between the bridging protocol and the token holder. This guarantees that upon redemption of the wrapped token, the original token will be returned. The value of the wrapped token is determined by the underlying asset value minus the risk associated with the bridging protocol failing to fulfill the debt. Presently, the market often misprices wrapped tokens, valuing them at the same level as the underlying asset. Consequently, when a trusted bridge fails to honor the debt, such as in the event of a hack resulting in the loss of the underlying asset, the economic impact is significant.

The IBC protocol solves the issue of secure cross-chain communication, but it was originally only implemented for Cosmos SDK chains using the Tendermint consensus.

**Picasso fixes this.**

## IBC Protocol - a trust-minimized standard for cross-ecosystem communication

The IBC protocol is an open-source and permissionless protocol designed to facilitate the authentication and transport of data, including message passing, tokens, NFTs, and more, between two networks and the applications within them. IBC is adaptable and can be implemented across different blockchains, networks, or state machines. In pursuit of its core mission, significant strides have been made in extending IBC implementations via Picasso, to facilitate decentralized and non-custodial cross-chain transactions. 

:::info
The IBC protocol is maintained by the [Interchain Foundation](https://interchain.io/) and currently boasts three mainstream implementations: one written in [Golang](https://github.com/cosmos/ibc-go), another in [Rust](https://github.com/cosmos/ibc-go) and the third in Solidity. Currently, Picasso is the only Cosmos SDK chain on mainnet communicating with blockchains using the Rust and Solidity implementations.
:::

Introduced in 2019, IBC was integrated into the Cosmos SDK in 2021. Presently, over 107 chains are interconnected, with IBC facilitating a volume of more than 5 billion USD as of December 2023, and [over 5 million token](https://mapofzones.com/home?columnKey=ibcVolume&period=24h) transfers between IBC-connected chains. These statistics underscore IBC's resilience over time and its ability to evolve its protocol to meet the needs of users and app-chains. Its widespread adoption and continued relevance within the Cosmos ecosystem have also garnered attention in the broader crypto community. 

The requirements of implementing IBC are not restrictive in terms of the type of consensus algorithm, allowing it to connect various types of blockchains, such as those based on Cosmos typically powered by Tendermint/CometBFT, Ethereum-like networks, and Solana as well. Picasso's innovation is not limited to Cosmos but rather actively leveraging and extending the IBC protocol beyond Cosmos, pushing its boundaries beyond its original scope. **Picasso's approach involves [establishing the IBC framework as the industry standard](https://medium.com/@Picasso_Network/ibc-as-the-end-game-of-bridging-a-comparison-analysis-on-trust-dcc01e0d9377) for cross-ecosystem communication.**

IBC supports asset transfers (fungible tokens, non-fungible tokens), generic message passing, cross-chain contract calls, cross-chain fee payments, interchain collateralization and more in a trust-minimized manner. The trust-minimized condition of IBC is due to the fact it is:

- built upon light clients that communicate with each other and updates the state of a counterparty chain. These are lightweight versions of blockchain nodes that can verify specific information on counterparty chains without downloading the entire blockchain history. This allows for secure verification of data and state on external blockchains
- typically used by Proof-of-Stake (PoS) blockchains which provide a high level of security, reducing the need to trust a single entity or centralized authority
- utilizing on-chain verification of transactions and messages. This means that the counterparty chain can independently validate the correctness of incoming messages and transactions using its own consensus rules, eliminating the need to trust external sources
- able to upgrade the state of a chain through sending finality proofs, other types of transactions and packets that can be verified
- employs mechanisms to prevent double-spending of assets across blockchains

For a more detailed technical explanation, please refer to the [IBC section](../technology/ibc.md).