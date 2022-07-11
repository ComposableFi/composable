# Overview

This pallet represents NFT, for the main usage in DeFi.

Allows creating NFT with of specified class, with owner, instance id and map of attributes.

Provides abstractions for typed and reference NFT design.

## fNFT

A fNFT instance account can be made the owner of financial positions by referencing these in specific protocols.
The original owner becomes an owner of the fNFT. 
The fNFT account becomes the owner of underlying position.

fNFT unifies interface possible around positions:

- limited proxied access to xTokens minted for positions shares
- maturity dates
- amount of tokens hold in position, locked and not locked
- mint, split and burn underlying positions
- ability to trade positions

**Example**

fNFT account can be use `account proxy` to delegate xTokens received for share to any other account to be used in  `democracy` for voting.


## XCMP

XCMP transfer will change the owner of  fFNT to sovereign account of sibling parachain where fNFT wash "transferred". 

## References

### NFT designs

- https://wiki.polkadot.network/docs/learn-nft
- https://github.com/open-web3-stack/open-runtime-module-library/tree/master/nft
- https://docs.metaplex.com/token-metadata/specification
- https://wiki.polkadot.network/docs/learn-proxies

### Financial

- https://fundamentallabs.substack.com/p/financialized-nfts-evolving-opportunities