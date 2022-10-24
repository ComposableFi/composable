# Automated Market Makers

To understand how AMMs work, we'll first take a brief detour to traditional finance (tradfi). To understand the whole topic, you'll need to be familiar with the following concepts:

- [Order Book](https://www.investopedia.com/terms/o/order-book.asp)
- [Spread](https://www.investopedia.com/terms/s/spread.asp)

[![What is a Market Maker and How do They Make Money?](https://img.youtube.com/vi/-zTHKcJEGe8/maxresdefault.jpg)](https://youtu.be/-zTHKcJEGe8)

## Constant Product Market Makers

In smart contracts, keeping track of a full order book is quite expensive, due to the large amount of data that needs to be stored, as well as the computational cost of ordering the book during trade execution. What the founders of [bancor](https://research.thetie.io/bancor-history/) realized, was that instead of an exchange being a platform for market makers, the exchange itself could function as a market maker, and do away with the order book. 

[![What is an Automated Market Maker? (Liquidity Pool Algorithm)](https://img.youtube.com/vi/1PbZMudPP5E/maxresdefault.jpg)](https://youtu.be/1PbZMudPP5E)

### Takeaways

We use AMMs and liquidity pools because order book-based DEXes cannot be properly done in smart contracts. Do note that AMMs are strictly less capital efficient and flexible compared to order book-based exchanges.

## Stableswap

AMMs can be used to trade any asset, however, we can optimize the Constant Product AMM algorithm with knowledge of the underlying assets `real` value. For example, stablecoins should always have similar values. Stableswap is an adaptation of the algorithm:

[![Curve - Math | DeFi](https://img.youtube.com/vi/GuD3jkPgPgU/maxresdefault.jpg)](https://youtu.be/GuD3jkPgPgU)


### Takeaways

- We can 'manipulate' the AMM algorithms to better suit the intended assets. Curve adds weights to attempt to always have stablecoins be priced around $1.

## Further Reading

- [Uniswap v2 Core](https://uniswap.org/whitepaper.pdf)
- [Uniswap v3 Core](https://uniswap.org/whitepaper-v3.pdf): Very cool, very complicated.