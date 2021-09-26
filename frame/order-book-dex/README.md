


### Matching engine

- receives on chain events about orders added/removed
- stores off chain all orders in price buckets (price) -> list(orderid ordered by timestamp)
- runs loop over sell/buy price buckets



## Examples

https://github.com/PacktPublishing/Blockchain-Development-for-Finance-Projects/blob/master/Chapter%208/contracts/orderbook.sol

## price level
- is ask can be reduce for some reason


## dutch auction

- price level is reduced with Timestamp

# book depths

- fixed with some percentage of oracle?
- any?

## Flash trade

one can buy/sell without having anything in pocket.

example - she can take collateral, put it to swap for deposit, and repay borrow


ASK: naive vs perforamnce?
ASK: Lending specific - only sell,

# Refs

[1]: https://docs.makerdao.com/smart-contract-modules/dog-and-clipper-detailed-documentation


yeah; we'll most like write a pallet for liquidations
:raised_hands:
1

9:52
which evaluates and attempts different strategies depending on available liquidity
