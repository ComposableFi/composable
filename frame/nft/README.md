# NFT framework
A pallet containing the most essential functionalities to add, and handle assets with NFT characteristics on the blockchain.

## Overview


The NFT framework pallet provides developers with fundamental tools including:
* Minting an asset into a wallet
* Edit, Customize and store an assets unique attributes
* Transfer of ownership between assets
* Burn assets from a wallet
* Inspect an assets attributes or owner

## Code Structure
A quick overview of the code structure, implementations and functions within them

### Variables
| Macro        | Name          | Description                                        |
|--------------|---------------|----------------------------------------------------|
| (crate) type | AccountIdOf   | The `AccountId` type used by the chain          |
| (crate) type | NFTInstanceId | u128                                               |
| struct       | NFTClass      | provided by `composable_traits` as `financial_nft` |




### Implementations
| Implementation       | Description                                                                    | Methods                                                                                                                        |
|----------------------|--------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------|
| inspect              | set of functions to inspect attributes and owner of an asset                   | `owner`<br> `attribute`<br> `class_attribute`                                                                                  |
| create               | creates a new class of objects                                                 | `create_class`                                                                                                                 |
| transfer             | transfer asset to a new owner                                                  | `transfer`                                                                                                                     |
| mutate               | set of functions to set attributes, mint, and burn assets                      | `mint_into`<br>`burn_from`<br>`set_attribute`<br>`set_typed_attribute`<br>`set_class_attribute`<br>`set_typed_class_attribute` |
| FinancialNFTProvider | full operation to create, set characteristics of and mint an NFT into a wallet | `mint_nft`<br>`mint_into`<br>`set_typed_attribute`                                                                             |


## Workflow
The function `mint_nft` provides us with all the operations to `mint_into` a wallet and store its unique `set_typed_attributes`.

For more customization we would use a combination of `create` to create ourselves a new instance of some class. 
Proceed with to `mutate` to add the specific attributes we want our asset to have.
And finishing up we `mint_into` our target wallet.




