# Composable Account Abstraction

The Virtual Wallet allows users to manage funds across multiple networks using a single native account on one designated chain, known as the "wallet" chain.

It's important to note that the virtual wallet is not a standalone contract; rather, it leverages the [CVM](../cvm.md) and [MANTIS](../mantis.md) to facilitate the seamless handling of user intentions across different blockchain domains while requiring just one signature from the user.

## Inventing the Virtual Wallet

### Problem

`Alice` only has an `Ethereum` wallet and is able to pay gas fees on ETH mainnet.

`Bob` only has a `Cosmos` wallet and is able to pay gas fees on the Cosmos Hub.

Alice wants to swap some `ATOM` for `ETH`.

Bob wants to swap some `ETH` for `ATOM`.

Bob and ALice are both satisfied with the price, and they are willing to exchange ATOM and ETH. However, the amount of `ATOM` Alice needs is more than what Bob has to offer.   

**How can we make this exchange happen?**

### General solution

To enable a secure and atomic exchange, ensuring that both Alice's and Bob's assets are available simultaneously on the same domain, we can employ a 3rd chain, referred to as the "Composable" domain. Here's how it can be achieved:

1. Escrow Tokens: Alice and Bob escrow their respective tokens on their source chains (Ethereum and Cosmos Hub). This process involves locking their assets in a smart contract or escrow mechanism on their respective chains.

2. Bridge Information: The next step is to bridge information about the escrow and its ownership from both Ethereum and Cosmos Hub to Composable. This information should include details about the locked assets, ownership, and any conditions required for the exchange.

**Atomic Exchange**: Once the information is successfully bridged to the Composable domain, an atomic exchange can be initiated. This exchange process should ensure that both Alice's and Bob's assets are released simultaneously on the Composable chain, enabling a secure and coordinated transfer.

**This mechanism resembles an IBC ICS-20 or Polkadot XCM reserve transfer.**

Upon the arrival of information about the escrowed tokens on Composable, users can engage in atomic token exchanges with each other. 

After the swap, users have various options:

- **Retaining Assets on Composable:** Users can choose to keep their assets on Composable, allowing them to settle their next intentions within the Composable ecosystem.

- **Moving to the Originating Chain:** Users can move their assets back to the originating chain where they have their primary wallet.

- **Moving to the Source Chain of Assets:** Users may decide to transfer their assets back to the source chain where the tokens were initially minted.

- **Moving to Any Other Chain:** Users also have the flexibility to move their assets to any other network of their choice.

In all these scenarios, users want to maintain control over their assets. To facilitate this, the concept of a CVM Executor is introduced, which acts as a cross-chain account. The CVM Executor plays a crucial role in enabling users to retain control over their assets while seamlessly managing them across different blockchain networks and domains.


### Account creation

When a user sends a message from their domain to another domain, the CVM creates an executor for each originating `signature + chain` pair. For example, if a user moves funds from Ethereum to the Composable domain via CVM, the system creates an executor to represent this particular `signature + chain` pair.

This architecture allows the user to send messages at the same time or in the next message, to execute various actions such as placing orders or initiating exchanges on behalf of the user. These actions are executed through any CVM executor owned by the user on any chain.

It's essential to note that CVM enforces strict security measures, ensuring that only the specific signature on the designated chain can issue funds management transactions. This mechanism maintains the sovereignty and control of the user's assets while allowing them to seamlessly manage their funds across different chains and domains.

**If a user has a native wallet on Composable or desires to authorise external accounts to oversee the management of their assets across the various chains where these assets have been sent, the following applies:**

*Configuring Proxy (Delegation) Accounts in the CVM Executor*

Users have the ability to initiate a CVM program to integrate a proxy or delegation account with their CVM Executor account. This enables a native account in Composable to (1) oversee and manage the user's funds, subject to predefined constraints, and (2) execute operations on behalf of the user's CVM Executor.

*Universal Compatibility*

This functionality is universally applicable across all networks where the CVM is operational, provided that standardized protocols for proxy or delegation exist. Users can leverage this feature to manage their assets consistently across compatible chains.

*Delegation to Multiple Origins*

Users retain the flexibility to delegate these responsibilities to multiple origins of CVM Executors.

**Why is the user's funds not showing up on their native wallet?**

Until native wallets on individual chains provide support for the CVM, users will need to make use of a CVM-specific wallet and accompanying dashboard. These specialized tools are designed to consolidate and display the user's assets across all chains where the CVM is operational.

**Does this mean the Virtual wallet is custodial?**

No, the Virtual Wallet is non-custodial. The instance of the CVM executor is created per user signature. Therfore, the funds are always in the CVM executor which is owned by the user or in flight over IBC.

CVM contracts are managed by cross chain DAO.

## Benefits of the Virtual Wallet 

The Virtual Wallet streamlines operations, reduces risks, enhances gas efficiency, and promotes increased liquidity usage. Additional benefits include:

1. **Opt for Local Atomic Exchange:** Instead of relying on multi-block swaps that may fail, it executes local atomic exchanges followed by cross-chain transfers, which have a lower likelihood of failure.

2. **Simplify Multi-hop Operations:** Replaces multi-hop transfers and multi-hop swaps with location-based operations on assets already accessible in wallets with delegated liquidity, such as through CoW orders.

3. **Flexible Fund Management:** Allows cross-chain asset management to adjust path prefixes, either expanding or reducing them when beneficial.

4. **Consolidate Cross-Chain Operations:** Bundles multiple small cross-chain operations into a single transaction for improved efficiency.


**The role of Solvers in MANTIS in an example use case for the virtual wallet**

Alice seeks to exchange her ETH for ATOM on Neutron.

Bob aims to exchange his ATOM for ETH on the Ethereum network.

Both parties escrow their input tokens to facilitate the exchange on Composable.

Solvers have identified that:

- Bob can receive ETH directly on Composable.
- Alice can obtain ATOM directly on Composable.

This approach offers a more secure and cost-effective exchange compared to permitting intermediaries to transfer and swap tokens on other chains. 
