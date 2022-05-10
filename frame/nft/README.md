# Pallet-NFT
A pallet containing the most essential functionalities to create, and manage assets with NFT characteristics.

## Overview
The NFT framework pallet provides developers with fundamental tools including:
* Minting an asset into a wallet
* Edit, Customize and store an asset's unique attributes
* Transfer of ownership from one user to another
* Burn asset from a wallet
* Inspect the attributes and owner of an asset

## Implementations
Trait implementations for Pallet-NFT are provided by composable_traits and composable_support

| Name                 | Description                        |
|----------------------|------------------------------------|
| FinancialNFTProvider | Handle AccountId                   |
| NFTClass             | Default ClassId type for NFT's     |
| Create               | Handle creation of assets          |  
| Inspect              | Inspect information of an asset    |
| Mutate               | Change an asset's information      |
| Transfer             | Facilitate transfer of ownership   |

## Extrinsics
| Name                      | Caller | Description                                                                                                                                         |
|---------------------------|--------|-----------------------------------------------------------------------------------------------------------------------------------------------------|
| create_class              | Admin  | Create a new class of assets, ensuring the class doesn't already exists                                                                             |
| owner                     | User   | Display the owner of an asset or vice versa                                                                                                         |
| attribute                 | User   | Display all assets identified by a given attribute                                                                                                  |
| class_attribute           | User   | Display all assets identified by a given class attribute                                                                                            |
| transfer                  | User   | Unless the asset can't be found, transfer given asset to a new owner                                                                                |
| mint_nft                  | User   | Unless `NFTCount` returns invalid values, `mint_nft` into a given wallet, using `mint_into` and define the type of asset with `set_typed_attribute` |


## Workflow
The function `mint_nft` provides us with all the operations to `mint_into` a wallet and set its `set_typed_attributes`.
We can then `transfer` ownership of the NFT we just minted to a given wallet.
