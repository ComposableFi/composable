# Digital Assets

Although blockchain can have many applications besides finance, the main use case is (and will most likely remain) decentralized finance. We express these assets using tokens, which are digital currencies managed by smart contracts. The most common token standard is `erc-20`, which describes how smart contracts need to manage tokens (such as how many tokens exist, can they be transferred, etc.)

An `erc-20` contract is very simple. Internally it contains a mapping of `addresses` to `balances`:


```
address -> balance // The `->` symbol indicates a mapping, meaning here that for a given address
                   // we can look up the balance 
```

It uses this mapping to see how many tokens any address has. When your wallet displays that you own 300 USDC, it sends a query to a node. The node then internally calls the `erc-20` smart contract of USDC, calling the `balanceOf` method. Internally the `balanceOf` looks up the balance in the mapping and returns that through the node to your wallet.

```
1. Wallet queries node, sending `erc-20.balanceOf(yourAddress)`
2. Node loads smart contract, then forwards query.
3. Smart contract is called with `erc-20.balanceOf(yourAddress)`
4. `yourAddress -> balance` lookup is executed.
5. Balance is returned from smart contract to node.
6. Node returns the balance to your wallet, which can then display the value.
```

This means that tokens are never *in* your wallet. Your wallet is just storing your address, and the tokens are just a number attached to your address specifically.

[![erc-20 tokens - Simply Explained](https://img.youtube.com/vi/cqZhNzZoMh8/maxresdefault.jpg)](https://youtu.be/cqZhNzZoMh8)

The difference between a coin and a token is a pedantic discussion. We prefer the term token, as they technically cover a wider range of [fungible assets](https://www.investopedia.com/terms/f/fungibility.asp). Some tokens are used for paying gas fees on the respective chains, such as `Ethereum`. For historical reasons, these are different than `erc-20` tokens, although Ethereum could choose to migrate and use `erc-20` tokens to pay for fees.

### Takeaways

Understand that tokens are not magical and that your wallet just manages your private key but doesn't actually `hold` the assets. 

## LP tokens and NFTs

Some tokens don't represent a currency, but more of a share of an asset, such as stocks. These are still fungible assets. The equivalent to a `stock` in crypto is an LP token, which represents fractional ownership of a larger pool of other tokens.

[How Liquidity Provider (LP) Tokens Work](https://www.gemini.com/cryptopedia/liquidity-provider-amm-tokens#section-lp-tokens-and-crypto-liquidity-providers)

NFTs are a different standard, but the same as `erc-20` tokens, except the total supply is always 1. These can be used to express non-fungible assets. An example of a non-fungible asset is a house. Even though we have housing markets, each house must be priced differently (different number of rooms, location, etc.).

[Non-fungible tokens (NFT)](https://ethereum.org/en/nft/)

### Takeaways

Tokens are financial instruments and can be used to express many different financial concepts. NFTs fall in the same category, even though they are now primarily used for artworks.

## Further Reading

- [erc-20](https://ethereum.org/en/developers/docs/standards/tokens/erc-20/)
- [erc-721](https://ethereum.org/en/developers/docs/standards/tokens/erc-721/)
- [cw20](https://docs.rs/crate/cw20/0.2.0)