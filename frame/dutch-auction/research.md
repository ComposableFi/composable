
## MakerDAO

- https://docs.makerdao.com/smart-contract-modules/dog-and-clipper-detailed-documentation

- https://docs.makerdao.com/keepers/the-auctions-of-the-maker-protocol

- splits collateral by pieces one by one put into auction

- auction time parameters depend on liquidity of collateral (these set by governance as collateral factor)

- see example of it in `clip.sol` of makerdao


## HydraDX

https://github.com/galacticcouncil/Basilisk-node/tree/master/pallets/exchange

- Intention to sell (a,b) and buy (b,a) are added during block
- Each block cleaned, so no data retained in block about intentions
- If exact matches found, than sell via OB
- If not exact found, sell remaining on AMM
- Can be used without AMM if set AMM allowance to low percentage or disable on runtime
