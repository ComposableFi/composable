# Pallet-NFT
A pallet containing the most essential functionalities to add, and handle assets with NFT characteristics on the blockchain.

## Overview
The NFT framework pallet provides developers with fundamental tools including:
* Minting an asset into a wallet
* Edit, Customize and store an assets unique attributes
* Transfer of ownership between assets
* Burn assets from a wallet
* Inspect an assets attributes or owner

## Code Structure
A quick overview of the code structure, implementations and extrinsics.

### Implementations
Trait implementations for Pallet-NFT are provided by composable_traits and composable_support

| Name                 | Implementation     | Description                       |
|----------------------|--------------------|-----------------------------------|
| FinancialNFTProvider | composable_traits  | Functionality to handle AccountId |
| NFTClass             | composable_traits  | Default ClassId type for NFT's    |
| Create               | composable support | handle creation of assets         |  
| Inspect              | composable support | inspect information of an asset   |
| Mutate               | composable support | change an assets information      |
| Transfer             | composable support | facilitate transfer of ownership  |

### Extrinsics
| Name                      | Caller | Description                                                                                                                                                                               |
|---------------------------|--------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| create_class              | Admin  | Create a new types of assets, ensuring the class doesn't already exists                                                                                                                   |
| owner                     | User   | Display the owner of an asset or assets of an owner                                                                                                                                       |
| attribute                 | User   | Display all assets identified by a given attribute                                                                                                                                        |
| class_attribute           | User   | Display all assets identified by a given class attribute                                                                                                                                  |
| transfer                  | User   | Unless the asset can't be found, transfer an asset to a new owner                                                                                                                         |
| mint_into                 | User   | Mints asset into a wallet `AccountIdOf` ensuring an instance of it doesn't already exist                                                                                                  |
| burn_from                 | User   | Unless the asset can't be found, `burn_from` an asset from a wallet `AccountIdOf`                                                                                                         |
| set_attribute             | User   | Unless the asset can't be found, set a `key` attribute of an asset to a given `value`                                                                                                     |
| set_typed_attribute       | User   | Unless the asset can't be found, set an encoded `key` attribute of an asset to a given `value`,                                                                                           |
| set_class_attribute       | User   | Unless the class can't be found, set a `key` attribute of a type of assets to a given `value`                                                                                             |
| set_typed_class_attribute | User   | unless the class can't be found Sets an encoded `key`attribute of a type of assets to a given `value`                                                                                     |
| mint_nft                  | User   | Mint `mint_nft` into a wallet, unless `NFTCount` returns invalid values, by calling `mint_into` and define the type of asset with encoded `key` and `value` calling `set_typed_attribute` |


## Workflow
The function `mint_nft` provides us with all the operations to `mint_into` a wallet and set its `set_typed_attributes`.
We can then `transfer` ownership of the NFT we just minted to a new wallet `AccountIdof`. 
Looking for the new `owner`'s collection we can find our asset.
