# Liquidations

A liquidation is a process that occurs when a borrower's collateralization ratio drops below a certain threshold. Once detected, the lending protocol will initiate a sequence of operations to liquidate the collateral of the user in order to obtain more borrow assets and stabilize the collateralization ratio.


#### When collateral ratio may drop?


Borrow price raised or collateral price dropped a lot. Borrower can add more collateral and return some borrow to make ration good again.


### Current Protocols

Lending protocols will often auction off collateral assets, often through an Order Book, or a Dutch Auction. Any account is able to bid/purchase the collateral from the lending protocol, often for an attractive, below market price. Liquidators are then able to to execute an arbitrage to earn a yield, by selling the collateral on a different exchange (often centralized).


Using off-chain liquidators has a number of strategic disadvantages. Liquidators are often anonymous and not invested in the protocol, which means that they are making (a big profit) while disregarding the community. They often collaborate with miners, as liquidations of collateral are a prime target for MEV.


Running liquidations through a decentralized exchange is possible, but the risk of frontrunning makes it economically infeasible on Ethereum. However within the Polkadot and Kusama ecosystem, using technology pioneered by HydraDX, we are actually able to automate and decentralize a liquidation.


### Hybrid Liquidations

By integrating with ~~HydraDX~~, liquidations become trustless, decentralized and transparent to the community. They are also more beneficial to the protocol, as we are able to get market price for the collateral. However as ~~HydraDX~~ is present on a different chain (~~Basilisk~~), we must account for different failure modes, and have fallback strategies. For that we propose a modified Dutch Auction model, which still incorporates a staking model to reward Angular token holders and ties liquidators into our ecosystem


#### Preferred Liquidators

Using a modified Dutch Auction model, we are able to grant certain accounts an advantage during auctions. Given the fictional auction over blocks 1 -> 2 -> 3 -> 4 -> 5, where the price decreases from 100 to 50 linearly for the collateral (100 -> 87.5 -> 75 -> 62.5 -> 50), we reach block 3 of the auction, allowing regular liquidators to purchase the collateral for 75. The preferred liquidator may choose to win that blocks auction, regardless of other offers, by paying the price of the previous block (87.5). Since the liquidator has a 6 second grace period, between block 2 and 3, they are able to execute a matching order on a DEX (say for 89), to pre-sell the collateral for a profit, reducing their risk.

Preferred liquidators may be configured on a per lending pool basis, and are subject to decentralized governance, allowing the community to evict malicious or inactive preferred liquidators.


*<iframe width="100%" src="https://viewer.diagrams.net/?tags=%7B%7D&highlight=0000ff&edit=_blank&layers=1&nav=1&title=dutch-auction-pl.drawio#R7Vlbb9owFP41eezk3Lg8tsDaaZ2G1Enr%2BmaS08RSEiPHgdBfP4c4JLahNwFh0hAP9ufjS75zzucTsNxJWt4yvIx%2F0BASy0FhablTy3Fsz3Gs6ovCjUTGtkQiRkKJtcADeQEJIokWJIRcMeSUJpwsVTCgWQYBVzDMGF2rZs80UXdd4ggM4CHAiYn%2BJiGPa3Tkoxa%2FAxLFzc42kiMpbowlkMc4pOsO5M4sd8Io5XUrLSeQVOw1vNTzvh4Y3R2MQcbfMwHT1eqq8O%2Fjpzz%2FvvjGnvJn90qussJJIR9YHpZvGgYgC68rIkUvSHCek8Byb2KeJgKwRZPRIguh2gSJHoSCPjmZMh7TiGY4mbXojXlueYhqageQT3ELNAXONsKAQYI5WamewdLB0c5uN3VOidjCQTIaPekIGYojpC6Q04IFIOd0eXx9Gc%2FT1uGYRcCNdUSj8zAttPXSBzzmGB77RVIwvSbIvMcLkY7CQ5CTF7zYDiHVeTghUVZ5VjgDmABWwDgR4X8tB1IShgfc9lpAyXSUm7ZJUC0P5XG8rDD7URp9g0bb4FCN7HVMODwscVCNroXcqUweDOt3PHKpRqQWoOtWdwYSijuSM0AnomhgUGSKQ88U2YOeORoaHLmXxtFOsPriaGRw5F0aR67TM0djgyP%2F0jjyUM8cNSVhh6TrgFNmECUqrWXVLNKkNmivte2VOKc54YRW19uCck7TPfcepxqhtOAJyWCyqzPRcVh2bS1dbZNm96w0uwbNBsPHrgmhJPyxsRbtP1X7i6t95OC07FhON53OHJiohbaFTI1lgpBHeaZtp17Yb7rtUtve5i231jXiO2K0LgLfvlxPXfP6anCNP1nzast4%2FnlrXtszQvLnIgcmaHHQnJHggupfmT0XXwDbZgU8nT32ful4eoW352YenlUOzTL4bHJon0gO0cXJYRONp9bDXaGnF8cfVURbk0QHnVkSzTePWQlBwStJvDPfZXvTw8G%2Foofma8r%2FsufoeT48U5rr2fnZNPfG2nWk3zMH0lyECd50zJaVQX74wI62T1PyHzqXYT9U7EWjPsFxNcd8S%2B1ozp506UtzRr1rjui2v%2BzX%2FLd%2FkLizvw%3D%3D"></iframe>*
*Angular: Liquidations Diagram - [Source](https://viewer.diagrams.net/?tags=%7B%7D&highlight=0000ff&edit=_blank&layers=1&nav=1&title=dutch-auction-pl.drawio#R7Vlbb9owFP41eezk3Lg8tsDaaZ2G1Enr%2BmaS08RSEiPHgdBfP4c4JLahNwFh0hAP9ufjS75zzucTsNxJWt4yvIx%2F0BASy0FhablTy3Fsz3Gs6ovCjUTGtkQiRkKJtcADeQEJIokWJIRcMeSUJpwsVTCgWQYBVzDMGF2rZs80UXdd4ggM4CHAiYn%2BJiGPa3Tkoxa%2FAxLFzc42kiMpbowlkMc4pOsO5M4sd8Io5XUrLSeQVOw1vNTzvh4Y3R2MQcbfMwHT1eqq8O%2Fjpzz%2FvvjGnvJn90qussJJIR9YHpZvGgYgC68rIkUvSHCek8Byb2KeJgKwRZPRIguh2gSJHoSCPjmZMh7TiGY4mbXojXlueYhqageQT3ELNAXONsKAQYI5WamewdLB0c5uN3VOidjCQTIaPekIGYojpC6Q04IFIOd0eXx9Gc%2FT1uGYRcCNdUSj8zAttPXSBzzmGB77RVIwvSbIvMcLkY7CQ5CTF7zYDiHVeTghUVZ5VjgDmABWwDgR4X8tB1IShgfc9lpAyXSUm7ZJUC0P5XG8rDD7URp9g0bb4FCN7HVMODwscVCNroXcqUweDOt3PHKpRqQWoOtWdwYSijuSM0AnomhgUGSKQ88U2YOeORoaHLmXxtFOsPriaGRw5F0aR67TM0djgyP%2F0jjyUM8cNSVhh6TrgFNmECUqrWXVLNKkNmivte2VOKc54YRW19uCck7TPfcepxqhtOAJyWCyqzPRcVh2bS1dbZNm96w0uwbNBsPHrgmhJPyxsRbtP1X7i6t95OC07FhON53OHJiohbaFTI1lgpBHeaZtp17Yb7rtUtve5i231jXiO2K0LgLfvlxPXfP6anCNP1nzast4%2FnlrXtszQvLnIgcmaHHQnJHggupfmT0XXwDbZgU8nT32ful4eoW352YenlUOzTL4bHJon0gO0cXJYRONp9bDXaGnF8cfVURbk0QHnVkSzTePWQlBwStJvDPfZXvTw8G%2Foofma8r%2FsufoeT48U5rr2fnZNPfG2nWk3zMH0lyECd50zJaVQX74wI62T1PyHzqXYT9U7EWjPsFxNcd8S%2B1ozp506UtzRr1rjui2v%2BzX%2FLd%2FkLizvw%3D%3D)*


Here, the actor (possibly an off chain worker), first executes the sell side of the collateral. After a successful sale, it uses the preferred liquidator privilege to execute the second half of the arbitrage. If H1 were to fail, the Dutch auction simply continuous using the regular liquidators, ensuring that the collateral is successfully liquidated.


TODO:

* after liquidation happened, where should go amount? should account be erased from debt fully? what percentage to amount can be returned him back? is it possible to partially liquidate? kind of sold out part? different type of auction (acala like)? global vs local liquidation . if sell off did not covered debt, should take amount from stability fee? how to punish operators?
