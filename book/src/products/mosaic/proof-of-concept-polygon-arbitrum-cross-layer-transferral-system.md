# Proof of Concept: A Cross-Layer Transferal System

---

We have successfully initiated our Layer 2 - Layer 2 asset transferal system (Mosaic) with a Proof of Concept (PoC) linking Polygon, Fantom, Arbitrum, the Avalanche C-Chain, Moonriver, and the Ethereum mainnet (layer 1). 

Polygon, Arbitrum, and Avalanche are leading L2/scalability solutions for Ethereum, with all being used by many of the largest DeFi protocols. For instance, [nearly 100 protocols](https://web.archive.org/web/20210725112237/https://www.block123.com/en/feature/polygon-matic-network-list/) have built on Polygon including MakerDAO's [DAI](https://makerdao.com/en/) (with [MakerDAO](https://makerdao.com/)being the second largest DeFi protocol at the time of writing, having over $13.5 billion in TVL, or total value locked), [Chainlink](https://chain.link/), and [Aavegotchi](https://aavegotchi.com/). Arbitrum has been incorporated into major DeFi protocols like [Bancor](https://bancor.network/) (currently the 13th largest DeFi project in terms of TVL), [Augur](https://augur.net/) and others. Avalanche similarly has an impressive [343 protocols](https://www.avax-projects.com/) built along it at the time of writing.

Polygon is a sidechain scaling solution that describes itself as “a protocol and a framework for building and connecting Ethereum-compatible blockchain networks… supporting a multi-chain Ethereum ecosystem”.

Arbitrum is also a major player in the space. Arbitrum’s rollup (with its mainnet released on May 28th) is an Optimistic Rollup (OR) that can scale any Ethereum smart contract, optimally suiting projects open to public participation. Arbitrum is also working on two other scalability solutions which are not yet released, Channels and Sidechains, to better suit additional types of projects.

[Avalanche's C-Chain](https://support.avax.network/en/articles/4058262-what-is-the-contract-chain-c-chain) is a blockchain compatible with Ethereum by leveraging the Ethereum Virtual Machine (EVM). This chain is designed for applications that require total ordering. It can handle thousands of transactions per second with sub-second finality at a much lower cost than the mainnet, allowing it to act as a scaling solution. The Avalanche network is also more attack-resistant and decentralized, largely due to its [Snowman](https://support.avax.network/en/articles/4058299-what-is-the-snowman-consensus-protocol) consensus protocol.

---

## The Initial Phase of the PoC: The Arbitrum-Polygon Cross-Layer Transferal System

Due to the pressing nature of resolving cross-layer interoperability, we first released the L1 vault solution for moving between different L2 scaling solutions (initially, Polygon and Arbitrum) as well as the Ethereum mainnet (layer 1, or L1). We also added compatibility with the Avalanche C-Chain, linking this network to those already connected to Mosaic. Effectively, the user is able to move a specific digital asset between different L2s as well as L1 without requiring a bridge between these solutions.

Our POC is intentionally simple and guarded in order to gather data on our idea that liquidity providing on L1, as a means for automated L2 transfer liquidity providing, and prove that it is a profitable model, as we [previously illustrated](https://0xbrainjar.medium.com/introducing-mosaic-tackling-cross-layer-2-liquidity-provisioning-through-delivering-a-new-means-of-1c1edb8691df) using a liquidity simulation engine (LSE) and test trade data.

This POC is designed to test our dynamic fee for single-sided stakers, as the first component of our full solution for Mosaic. 

As announced by our Head of Product 0xBrainJar, this [Proof of Concept (PoC) of Mosaic](https://0xbrainjar.medium.com/introducing-mosaic-tackling-cross-layer-2-liquidity-provisioning-through-delivering-a-new-means-of-1c1edb8691df) will involve a layer 1 vault which will facilitate liquidity provisioning across different layer 2 farms (for Arbitrum and Polygon). This provides an entirely new and potentially valuable means of generating yield (i.e. cross-layer liquidity provisioning), and also provides the liquidity necessary to facilitate cross-layer transactions.

Once this L1 vault went live, it had a cap of $3 million in total value locked (TVL) for three days allowing users to provide liquidity for subsequent cross-L2 transactions. Users could add liquidity into this vault to support the transactions of other users between Arbitrum, Polygon, and mainnet. The accepted assets were WETH and USDC.

After this period, we paused the liquidity provisioning into this vault, and then moved part of the liquidity from the L1 pool into each of the L2 vaults (on Polygon and Arbitrum).

From there, the POC went live, and users were be able to swap between Polygon and Arbitrum and L1. This allows for:

- Cross L2 swaps
- Fast exits to L1 from Polygon
- Fast exits to L1 from Arbitrum

When a user initiates a transfer, the relayer catches the lock event and call the release operation on the destination layer. If this is not successful, the user can then claim what they locked on the source layer.

The fee associated with this process is a dynamic fee ranging from 0.25–5%. It has a linear curve until the trade size is greater than 30% of the available liquidity on the destination layer, and then it becomes a flat 5% fee. These fees are then distributed to L1 liquidity providers. Users will be able to monitor in real time their earnings as L1 liquidity providers.

Fees will be monitored in real time and displayed in the user interface in USDC. However, we will also be converting these fees and distributing them in LAYR tokens, once our token generation event (TGE) occurs.

Furthermore, the Composable team will be rebalancing the liquidity in the system so as to ensure proper liquidity distribution throughout the duration of the PoC. Seeding the liquidity to kickstart this PoC will be done via Composable's multisig wallet address at 0xD991c4dE26156fc1854ad3f3Ef3104Dda7F27bAb.

Additionally, users who swap from Arbitrum or L1 to Polygon via our bridge will also each receive 0.05 MATIC tokens (the native token of Polygon, provided by their team). This ensures they are able to complete transactions along that L2 solution. 

---

## Future Additions for Cross-Layer Interoperability

This is just the beginning piece of what will become an extensive cross-layer interoperability system to come from Composable Finance, serving to resolve a major pain point in DeFi: moving assets between protocols that make use of different scaling solutions.

The PoC is just the first piece of the entirely composable cross-layer system that is Mosaic, with the addition of the Avalanche C-Chain already implemented. The immediate vision is for Mosaic to provide cross-L2 liquidity services to projects in this space; we aim to ultimately build a liquidity directing system to move liquidity as necessary around the L2 space. We aim to offer these services to be leveraged by other projects in DeFi, mutually benefitting Composable, projects on L2s, other L2-L2 solutions, and DeFi users by ensuring a seamless and symbiotic space free of liquidity limitations.

Thus, the Mosaic PoC is an essential foundation for future developments, in addition to providing real-world transactional data that we will feed into our Liquidity Simulation Engine (LSE), which is working to determine the requirements for providing liquidity for cross-layer transactions. If you want to learn more about the technical details of this simulation and how it provides a case for the potential significant value of cross-layer liquidity provisioning, this information is included in the previously mentioned [article by 0xBrainJar](https://0xbrainjar.medium.com/introducing-mosaic-tackling-cross-layer-2-liquidity-provisioning-through-delivering-a-new-means-of-1c1edb8691df).

Through this structure, the Mosaic PoC will act as a fully experimental approach for us to gather information to augment our existing simulation results. This will be critical in providing us with more realistic data to help us determine the best approach to take in resolving the liquidity management hurdle that has arisen across L2. With this launch of our cross-layer transferal infrastructure’s PoC, we will have the real-world data needed to further refine our model and determine the optimal means of liquidity balancing across L2s.