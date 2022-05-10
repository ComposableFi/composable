# NFT
## Overview
Pallet-NFT provides basic non-fungible token functionality.
In the following we will do a quick recap of what makes NFTs special followed by how that is reflected in Pallet-NFTs code.
And finally, we will discuss two use cases.



## Non-Fungible Tokens
NFTs (Non-fungible-Tokens) are different from fungible ones, like precious metals or fiat currency in several ways.

NFTs are:
* Uniquely Identifiable
* Represent ownership of a unique asset
* Can be minted independently
* Enable special ownership functionality

In concept, they became popular as a way to tokenize unique, collectible items.
Essentially the owners' public key acts as a proof of authenticity and their private key as a proof of ownership.



## Extrinsics
| Name                      | Caller | Description                                                                                                                                         |
|---------------------------|--------|-----------------------------------------------------------------------------------------------------------------------------------------------------|
| create_class              | Admin  | Create a new class of assets, ensuring the class doesn't already exist                                                                              |
| owner                     | User   | Display the owner of an asset or vice versa                                                                                                         |
| attribute                 | User   | Display all assets identified by a given attribute                                                                                                  |
| class_attribute           | User   | Display all assets identified by a given class attribute                                                                                            |
| transfer                  | User   | Unless the asset can't be found, transfer ownership of a given asset to a given wallet                                                              |
| mint_nft                  | User   | Unless `NFTCount` returns invalid values, `mint_nft` into a given wallet, using `mint_into` and define the type of asset with `set_typed_attribute` |



## Traits
The implementations to handle NFT traits used in Pallet-NFT are provided by composable_traits and composable_support.
Composable_traits provides us with the traits for FinancialNFTProvider and NFTClass.
Frame_support adds traits to `Create`, `Inspect`, `Mutate` and `Transfer` NFTs.



## Workflow
With the nature of NFTs in mind, it becomes imperative to have a straightforward implementation aiming to accommodate their traits.
We achieve this by having multiple methods to define distinct types and characteristics in an asset.

Ownership being a core concept in NFTs makes the presentation and trading of ownership a vital part of how we interact with them.
Showing off assets as well as the transfer of ownership is facilitated by Pallet-NFT.

The extrinsic `mint_nft` provides us with all the operations to `mint_into` a wallet and set its `set_typed_attributes`.
We can then `transfer` ownership of the NFT we just minted to a given wallet.



## Use Cases
Pallet-NFT is a module for integrators to make their own NFT applications.
Let's walk through at least two possible applications. One will be an NFT-Art-Gallery and the other a ticket booth.

---

### Art Gallery
An artist would mint their artwork into a wallet.
They proceed to give each artwork their respective traits, for this example let's assume we have set names and prices.
Now we are only a few steps away from our first NFT-based art sale.

Taking advantage of the framework's complete set of tools, an artist could:
* Host a private gallery of minted artwork
* Make their artwork searchable via recurring traits (style, color palette, materials, history)
* Transfer ownership of an art piece to a buyer

---

### Ticket booth
Moving on to the ticket booth.
An organizer could mint a number of NFTs representing access to an event.
By defining traits like seat numbers or levels of access, we could streamline the planning process of an event.
With each ticket having a unique digital signature and tracking its own history on the blockchain, we can make sure:
* The ticket is valid and was created by a licensed party
* If the ticket changed hands, and to whom
* To what and where the attendee has access to
