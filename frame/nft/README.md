# Overview

This pallet represents NFT, for the main usage in DeFi.

Allows creating NFT with of specified class, with owner, instance id and map of attributes.

Provides abstractions for typed and reference NFT design.

## fNFT

fNFT instance account can be made the owner of financial positions by referencing these in specific protocols.
The original owner becomes an owner of fNFT. 
fNFT account becomes owner of position.

fNFT unifies interface possible around positions:

- limited proxied access to xTokens minted for positions shares
- maturity dates
- amount of tokens hold in position, locked and not locked
- mint, split and burn underlying positions
- ability to trade positions

**Example**

fNFT account can be use `account proxy` to delegate xTokens received for share to any other account to be used in  `democracy` for voting.

## References

- https://wiki.polkadot.network/docs/learn-nft
- https://github.com/open-web3-stack/open-runtime-module-library/tree/master/nft
- https://docs.metaplex.com/token-metadata/specification
- https://wiki.polkadot.network/docs/learn-proxies
