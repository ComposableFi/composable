# TriCrypto, aToken, and SLP PoC Expansions

---

## Summary

We have added compatibility to Mosaic to allow for the cross-layer transferal of receipt tokens from the Curve, Aave, and SushiSwap platforms. These liquidity provider (LP) tokens are termed TriCrypto/aTriCrypto, aTokens, and SushiSwap LP (SLP) tokens, respectively.

By allowing users to swap these tokens across our Mosaic offering, they can be moved between any linked layers (currently, that includes Polygon, Arbitrum, the Avalanche C-Chain, and mainnet).

The benefit of adding LP tokens in particular to Mosaic is that LP tokens have many use cases in DeFi; in addition to being cashed out for their underlying LP stakings plus any accumulated rewards, LP tokens are often used as a means of collateralizing loans or participating in other dApps. Thus, we have allowed some of the most popular LP tokens to be integrated into Mosaic to allow for maximum user opportunities and functionality.

For each of these three token types that we added, we re-opened LPing for our cross-layer asset transferal system for an additional 3 days, this time leveraging Curve pools to provide ample liquidity with improved fees. After this 3 day LPing period into our L1 vaults, the LP vault deposits closed, and transfers of these LP tokens are now enabled across Mosaic.

---

## Improving our PoC LPing Structure

Composable is continually striving to discover the best way to optimize cross-layer liquidity provisioning (LPing) while also optimizing fees. These metrics are a key focus of our research related to [Mosaic](https://mosaic.composable.finance/) and its Proof of Concept (PoC), the [Polygon-Arbitrum Cross-Layer Transferal System](https://composablefi.medium.com/the-launch-of-our-polygon-arbitrum-cross-layer-transferral-system-a-novel-proof-of-concept-b4cfc8cf0023).

Through the research endeavors here at Composable Labs, we have developed a mechanism for liquidity provisioning across L2s that our simulations have shown to deliver high liquidity with increased rewards distributed to liquidity providers (LPs). This mechanism involves two [Curve Finance](https://curve.fi/) pools (one on Polygon and one on Arbitrum), which are monitored for exchange volume activity. The Composable software development kit (SDK) automates and simplifies token movements between these pools to abstract away the constant manual tracking and shifting of liquidity. This allows users to have a vastly simplified cross-layer transaction experience, where liquidity is not a concern, and LPs are amply rewarded - precisely the win-win situation Composable seeks to deliver. 

**Through our simulations, we project that this new active strategy will have additional returns: around a 4% APY or much higher (towards the theoretical max of >1,100%), as compared to a passive LPing strategy that would show lower returns.**

---

## Current Limitations in our Cross-Layer LP Model

Our first model for cross-layer LPing is incredibly promising; currently the performance in our vaults for wETH is a 37% APY, and 17% for USDC. This model involves a farming vault on L1 supporting cross-layer transactions, as explained in more detail [here](https://0xbrainjar.medium.com/introducing-mosaic-tackling-cross-layer-2-liquidity-provisioning-through-delivering-a-new-means-of-1c1edb8691df).

Yet despite this potentially lucrative new opportunity for cross-layer LPing, we know we can further improve this model and have been exploring options through Composable Labs. In particular, in the initial model, LPs would have to be constantly manually reshuffling liquidity across Polygon and Arbitrum. 

For example, take an LP that is 100% allocated to the Arbitrum pool when they discover that the revenue in another pool is increasing and will likely soon exceed their current income. Revenues vary over time and nothing is certain, but they decide to reallocate their contributions to 50/50 between the pools initially. To do this, they need to un-LP on Polygon and move the funds to L1 Ethereum waiting for the exit time pass. Then, they must move the funds to Arbitrum. Finally, they need to deposit into the Curve pool on Arbitrum, receiving the corresponding LP tokens. But, then it is likely that the revenue ratios between the two pools will change again shortly thereafter and a new reallocation will be necessary, requiring the lengthy, manual, error-prone process to be repeated.  

Enter the Composable SDK, automating and simplifying the token movement. This abstraction enables us to build even more powerful financial stacks and strategies, as presented here. Our alpha strategy seamlessly moves LP tokens between layers and chains with no need for concern or involvement from the user.

The LP simply inputs their exact strategy parameters (the rules governing when LP tokens are moved) and Composable’s technology takes care of the rest.

---

## Composable’s Layer 2 Alpha Strategy Overview

Before presenting our Layer 2 strategy, consider a passive investment strategy shown in Fig. (1) on data collected on-chain in the time period August 26th 2021 to September 26th 2021. Funds are allocated upfront in some ratio between the pools (the vertical black line shows a 50/50 allocation). We find the following returns are obtained on one particular snapshot of data (keep in mind that this curve below is time-dependent, we just show it at one point in time):

