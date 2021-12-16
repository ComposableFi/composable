# Overview

The exchange allows placing buy and sell orders at specific price levels, or at market level. The market level price can be provided by a combination of `pallet-oracle` and the future AMM DEX

Here is we design cross chain DEX. It will have interfaces like if it is on chain for pallets, but token swaps managed asynchronously by parachain (bridges). This pallet has only API to be called from bridge callbacks, not calling it.

Our DEX represents SELL side of traditional OB.

## What it is about?

First, what is exchanges of tokens across change?

It is based on protocol of token transfer, where A token is trusted(or proven) to be burn on A and minted on B.

Exchange, when A burns token x and mints y, and B mints x and burns y, and there is data sharing to agree on rate.

### DEX based liquidation

Sell the collateral on the DEX for the best price possible once the collateral passes some price point(collateral to borrow factor). Optimal is return back obtain at least the lent out coin(borrow principal) as return value from DEX.

External exchange is a trusted order book based exchange by trusted account id.

Fast it that there are up to few blocks allowed to liquidate.

Can be faster if untrusted, we will trust agent to burn amount.

For untrusted actors, more slow and complex schemas are needed.

Untrusted user must transfer borrow currency and buy collateral. There are [hash time locked swap][1](requires prove) and [reserver transfer via polkadot relay][2]. (they actually trust some third party consensus). And bridge some deposit first.

Important - assuming our parachain to be anemic - so it set states and allows  other to read that, not directly send message.

So that proffered account is of same level of trust as usual for now.

### Links

[1]: https://research.csiro.au/blockchainpatterns/general-patterns/blockchain-payment-patterns/token-swap/
[2]: https://medium.com/polkadot-network/xcm-the-cross-consensus-message-format-3b77b1373392
[3]: https://wiki.polkadot.network/docs/learn-bridges
