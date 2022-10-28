# Single-Sided Staking

Mosaic allows for ‘single-sided staking’, where users earn yield by providing liquidity for one type of asset, in 
contrast to liquidity provisioning on AMMs which requires a pair of assets. Mosaic’s rebalancing mechanism can move 
liquidity across chains using bridges, thereby allowing users to gain from transfers outside of the chains they 
originally provide liquidity on. 

Active liquidity also enables single-sided staking: active liquidity is provided when a user makes a request to transfer
an amount of tokens greater than the available liquidity and another user provides liquidity for that transfer. This can
be compared to the way orders are filled on TradFi exchanges, where orders are seen and then filled only after being 
specifically identified and met by a provider.


## Chains and Assets Supported on Day One

The chains/layers supported by Mosaic Phase 2 are as follows, enabling for active liquidity provisioning across these 
platforms:

* [Ethereum Mainnet](https://ethereum.org/en/)
* [Avalanche](https://www.avax.network/)
* [Fantom](https://fantom.foundation/)
* [Polygon](https://polygon.technology/)
* [Moonriver](https://moonbeam.network/networks/moonriver/)
* [Arbitrum](https://offchainlabs.com/)


### Compatible Assets

The assets that will be initially compatible with cross-layer/cross-chain transfers on Mosaic Phase 2 are as follows:

* USDC
* DAI
* USDT
* WETH
* MIM
* FRAX