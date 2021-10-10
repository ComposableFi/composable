# Overview


The exchange allows placing buy and sell orders at specific price levels, or at market level. The market level price can be provided by a combination of pallet-oracle and the future AMM DEX


## Liquidation


Used to liquidate collateral from `lending` protocol.  Need to liquidate because of bad collateral factor. It should be sold on exchange as soon as possible.



### DEX based liquidation

Sell the collateral on the DEX for the best price possible once the collateral passes some price point(collateral to borrow factor). Optimal is return back obtain at least the lent out coin(borrow principal) as return value from DEX.

Has pallet call extrinsic to allow users to liquidate

Exchange is a trusted order book based exchange by trusted account id.

Fast it that there are up to few blocks allowed to chosen trusted account id to liquidate.

External calls can be done by actor from other chain to liquidate.

Because of need to be fast and trusted, we will trust agent to burn amount.

After these blocks passed, liquidation should happen on chain. So that actors, untrusted, deposit borrow and exchange collateral.

Untrusted user must transfer borrow currency and buy collateral. There are [hash time locked swap][1](requires merkel prove) and [reserver transfer via polkadot relay][2]

### Initial State

- collateral transfer into on chain internal for DEX order

- some borrow amount on off chain order book DEX


## Final State

- collateral burnt on local chain

- borrow minted on local chain

- collateral minted on remote chain

- borrow burnt on remote chain


### Locked external DEX

- transfer currency to dex account derived from dex order id with 1 prices with prefered luidator
- after 3 blocks passed, only one default prices for anybody is passed.
- pre feed
- lock that amount for

### External OB DEXes



### Links

[1]: https://research.csiro.au/blockchainpatterns/general-patterns/blockchain-payment-patterns/token-swap/
[2]: https://medium.com/polkadot-network/xcm-the-cross-consensus-message-format-3b77b1373392
