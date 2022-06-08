# Foreign Assets

Angular lending can operate lending pools of any asset. There are a few requirements before a foreign asset (token originating from another chain) can be incorporated.

1.  The token must be bridgeable (XCM, IBC).
2.  It must have an AssetId assigned.
3.  Pricing data must be available from [Apollo](https://composablefinance.atlassian.net/wiki/spaces/COM/pages/33194147/Architecture+Overview#Oracles).

One of the privileged addresses may then create and configure the lending pool. Entire process is possible through extrinsics, avoiding the need for runtime upgrades.

### **Incentivizing** Usage

The source chain of the foreign asset may wish to incentivize the usage of their stablecoin or collateral by airdropping tokens, to increase the APY. Angular issues an LP token for each lending pool, which may be used to retroactively compute pool participation for airdrop purposes. A locked-staking pallet could also be used to distribute funds in a decentralized manner.
