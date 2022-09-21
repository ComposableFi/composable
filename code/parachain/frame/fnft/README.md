# fNFT
This pallet allows creating financial NFTs(fNFTs) with specific attributes and provides abstractions for typed and 
reference NFT design.

---

## Overview

Pallet fNFT provides an implementation of NFT's as proxied references for a users staking positions.

## Workflow

User facing mutating API is provided by other pallets.
Once liquidity pools have been configured and funded, fNFTs admined by the owner of the pool are minted and serve as 
proof of a users position. Users are able to create pools themselves, rewarding them with fNFTs in the same manner.
We can use these fNFTs to: 

* Transfer ownership of a position without leaving it
* Take part in protocol governance
* Represent an amount of claimable tokens



## References

### NFT designs

- https://wiki.polkadot.network/docs/learn-nft
- https://github.com/open-web3-stack/open-runtime-module-library/tree/master/nft
- https://docs.metaplex.com/token-metadata/specification
- https://wiki.polkadot.network/docs/learn-proxies

### Financial

- https://fundamentallabs.substack.com/p/financialized-nfts-evolving-opportunities