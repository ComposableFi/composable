## Liquidation

Used to liquidate collateral from `lending` protocol.  Need to liquidate because of bad collateral factor. It should be sold on exchange as soon as possible.

# Overview

- off-chain worker monitors `lending` markets for liquidation opportunities of borrow pairs.

- evaluates and attempts different strategies depending on available liquidity

- can puts risks assets into `dutch-auction`

## References

https://docs.makerdao.com/smart-contract-modules/dog-and-clipper-detailed-documentation

https://docs.makerdao.com/smart-contract-modules/system-stabilizer-module

https://docs.parallel.fi/dev/liquidation-1/liquidation

https://github.com/parallel-finance/parallel/blob/master/pallets/liquidation/src/lib.rs


