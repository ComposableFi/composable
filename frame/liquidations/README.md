# Overview

Used to liquidate collateral from `lending` protocol.  Need to liquidate because of bad collateral factor. It should be sold on exchange as soon as possible.

- off-chain bots monitor `lending` markets for liquidation opportunities of borrow pairs.

- uses different engines depending on available liquidity

- default engine is `dutch-auction`

## References

https://docs.makerdao.com/smart-contract-modules/dog-and-clipper-detailed-documentation

https://docs.makerdao.com/smart-contract-modules/system-stabilizer-module

https://docs.parallel.fi/dev/liquidation-1/liquidation
sad
https://github.com/parallel-finance/parallel/blob/master/pallets/liquidation/src/lib.rs
