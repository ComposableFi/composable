# [Overview](https://app.clickup.com/20465559/v/dc/kghwq-20761/kghwq-3621)

See overview of more business and architectural state of lending and all know features.

## Technical Reference

Lending = Self + [Liquidations](liquidations) + [Oracle](../oracle) + [Vault](../vault) + OCW

Market = Isolated Currency Pair + Configuration.

Oracle = only Pairs with Prices are allowed.

Vault = Vault provides and recalls liquidity into/from Lending via protocol. User must put amount of Borrow into Vault for Market creation. Lenders put borrow asset into Vault. Borrowers put collateral into Vault, get xCollateral, and use it to get borrow for collateral. Borrower pays rent for his position.

OCW(or anybody) watches for under collateralized Positions and sends them to Liquidations. Liquidator is rewarded with rent payed by borrower.

## Known limitations and constraints

As of now Lending does not handles cases when vault changes its decisions during single block.

Lending is executed after Vault. On block initialization. If can withdraw from Vault, than withdraws. If can deposit back, deposits maximal part available. If must liquidate, than deposit back all.

Borrowing is not allowed if we must liquidate (the vault is expecting the strategy to be liquidated) or if we market have enough funds or if have not enough collateral.

When repaying, we do not transfer the amount back to the vault. This is done within the pallet hooks on each block update.. It actually allows to use some deposit asset by other transactions in same block to borrow.
