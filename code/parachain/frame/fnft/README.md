# fNFT
This pallet allows the creation of [financial NFTs(fNFTs)] with specific attributes and provides abstractions for 
typed and reference NFT design.

---

## Overview

Pallet fNFT provides an implementation of NFTs as proxied accounts able to take ownership of assets and create 
controllers.

## Workflow

Other pallets will provide user-facing mutating API.
Once liquidity pools have been configured and funded, fNFTs are accessible and maintained by the pool owner.
Users can create liquidity pools themselves, rewarding them with fNFTs in the same manner.
The Owning account of the fNFT is set as a delegate for the fNFT `asset_account`. 
The `asset_account` delegates some functions to the owning account to act as a controller.

We can utilize these fNFTs to:

* Represent liquid assets / an amount of claimable tokens
* Act as a proxy account on the owners behalf
* Transferring positions taken / referenced by fNFT

## References

### NFT designs

- https://wiki.polkadot.network/docs/learn-nft
- https://github.com/open-web3-stack/open-runtime-module-library/tree/master/nft
- https://docs.metaplex.com/token-metadata/specification
- https://wiki.polkadot.network/docs/learn-proxies

### Financial

- https://fundamentallabs.substack.com/p/financialized-nfts-evolving-opportunities

[financial NFTs(fNFTs)]: https://github.com/ComposableFi/composable/blob/main/rfcs/0006-financial-nft.md