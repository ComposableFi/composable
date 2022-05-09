# Pallet-NFT
## Overview
Pallet-NFT provides basic non-fungible token functionality.
In the following we will do a quick recap of what makes NFTs special followed by how that is reflected in Frame-NFT's code.
And finally, we will discuss two use cases.

---

## Non-Fungible Tokens
NFTs (Non-fungible-Tokens) are different from fungible ones, like precious metals or fiat currency in several ways.

NFTs are:
* Uniquely Identifiable
* Represent ownership of a unique asset
* Can be minted independently
* Enable special ownership functionality

In concept, they became popular as a way to tokenize unique, collectible items. 
Essentially the owners public key acts as a proof of authenticity and their private key as a proof of ownership.

---

## Extrinsics 
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
---

## Traits
The implementations to handle NFT traits used in Pallet-NFT are provided by composable_traits and composable_support.
Composable_traits provides us with the traits for FinancialNFTProvider and NFTClass.
Frame_support adds traits to Create, Inspect, Mutate and Transfer NFTs.

## Workflow
With the nature of NFTs in mind, it becomes imperative to have a straightforward implementation aiming to accommodate their traits.
We achieve this by having multiple methods to define distinct types and characteristics in an asset.

Ownership being a core concept in NFTs makes the presentation and trading of ownership a vital part of how we interact with them.
Showing of(f) assets as well as transfer of ownership is supported by Pallet-NFT

The function `mint_nft` provides us with all the operations to `mint_into` a wallet and set its `set_typed_attributes`.

### Example



---

## Use Cases
Pallet-NFT is a module for integrators to make their own NFT applications.
Let's walk through at least two possible applications. One will be an NFT-Art-Gallery and the other a ticket booth.

### Art Gallery
An artist would mint their artwork into a wallet. 
They proceed to give each artwork their respective traits, for this example let's assume we have set names and prices.
Now we are only a few steps away from our first NFT-based art sale.

Taking advantage of the frameworks complete set of tools, an artist could:
* Host their own private Gallery of minted artwork
* Make their artwork searchable via recurring traits (style, color palette, materials, history)
* Transfer ownership of an art piece to a buyer

---

### Ticket booth
Moving on to the ticket booth
An organizer could mint a number of NFTs representing access to an event. 
By defining traits like seat numbers or levels of access, we could streamline the planning process of an event.
With each ticket having a unique digital signature and tracking its own history on the blockchain, we can make sure:
* The ticket is valid and was created by a licensed party
* If the ticket changed hands, and to whom
* To what and where the attendee has access to