# NFTs

Collateralizing NFTs is a hard problem for the following reasons:

1.  Low Liquidity
2.  Difficult Taxation
3.  Easy Pumping of Value


1\. makes it difficult to liquidate the collateral. Sufficient markets must exist for liquidators to sell the NFT, or for the protocol to automatically convert the NFT into borrow asset.


2\. and 3. are variations of the same problem. To hand out borrow asset, the protocol must securely determine the collateralization ratio. With NFTs, the previous sell price might be 100 ETH, but that does not mean that it can be liquidated for anything close to 100 ETH. The following attack illustrates that:


```plain
1. User A mints NFT XYZ.
2. User B purchases XYZ for 2 ETH.
3. User B sells XYZ to alias for 200 ETH.
4. User B uses alias to take out loan on XYZ for 50 ETH worth of USDC (collaterization ratio 200%).
5. User B defaults, earning 48 ETH net from the protocol.
6. Protocol is unable to liquidate pumped NFT, and loses 50 ETH worth of USDC.
```


### Possible Solutions


#### Advanced Market Places

NFT marketplaces supporting buy orders for specific NFTs can be used by the lending protocol to both liquidate and estimate the true price. The following conditions must hold:


*   Buy orders may not be arbitrarily cancelled, the lending protocol must be able to execute them after cancellation. This can be accomplished by delaying the cancellation.
*   Placing a buy order means locking the funds.


Advanced market places may accept LP tokens for the buy order to increase capital efficiency.


#### Tokenization of NFTs

High value NFTs may be tokenized, increasing liquidity. It requires the following conditions


*   Available AMMs for tokens.
*   Sufficient Liquidity


The lending protocol then becomes unaware that it is operating on NFTs, and instead provides pools for fungible tokens again. These would still be extremely high risk pools.
