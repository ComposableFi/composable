# Additional Vault Pallet Details

---

## Vault Features

Through this infrastructure, some of the overarching functionalities of Cubic include:

* Deposit and withdrawal management
* Governance
* LP token share management

These can be applied to innumerable DeFi strategies based on the particular vault and project it belongs to.


## User Incentives

Providing assets to a vault locks them, and grants the user xTokens (receipt tokens), which can later be used to return 
the original assets upon conclusion of the lock period. With these xTokens, users are given voting rights. xTokens can 
be seen as shares, which are diluted when more users join the vault, and then burned when users exit the vault. As seen 
in the other ecosystems, this opens up untapped opportunities for these receipt tokens which can be employed for use 
across various DeFi strategies such as collateral for lending protocols.


## Allocation Of Funds Strategy

The particular vault’s strategy determines the allocation of its funds. Initially, we will release a single strategy, 
which maps Account IDs to predetermined ratios (PerBill). Thereafter, each account will be able to withdraw up to the 
ratio * vault balance from the vault to be used. The actual strategy is performed by the pallet/smart contract 
associated with the account ID.


## Interfaces

Pallets using vault funds can depend on the traits exposed by the vault pallet to determine the requested fund 
allocation. Strategies in the Composable ecosystem should expose an interface for fund managers/councils to 
request re-balancing. The vault itself does not impose any interface for strategies, but instead exposes methods to 
relay data on how each strategy is performing. Thus, interfaces are specifically customized to the developer and their 
unique needs.


## Security

To ensure that funds can be rescued in the case of a negative event such as a theft or hack, vaults have different 
methods to halt functionality and return funds to LP token holders:



1. Pausing deposits and investments
2. Pausing deposits, investments, and withdrawals
3. Destroying the vault and returning all funds to the users

These extrinsics may only be called by the multisig account of the Picasso Network council, to ensure they are utilized 
appropriately.


Check out the standard implementation and extrinsics of Cubic 
[here](https://dali.devnets.composablefinance.ninja/pallets/vault.html).