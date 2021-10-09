# Overview


Exchange is a order book based exchange.



## Model

The exchange allows placing buy and sell orders at specific price levels, or at market level. The market level price can be provided by a combination of pallet-oracle and the future AMM DEX.


## Locked external DEX

- transfer currency to dex account derived from dex order id
- setup


### Lending Liquidation

Used to liquidate collateral from `lending` protocol.

Sell the collateral on the DEX for the best price possible once the collateral passes some price point(collateral to borrow factor). Optimal is return back obtain at least the lent out coin(borrow principal) as return value from DEX.

Has pallet call extrinsic to allow users to liquidate

