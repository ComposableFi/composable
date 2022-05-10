# Extrinsics

## Create Class

`create_class`

Create a new class of assets, ensuring the class doesn't already exist.
To be called as sudo.

## Owner

`owner`

Display the owner of a given asset or vice versa

## Attribute

`attribute`

Display all assets identified by a given attribute.

## Class Attribute

`class_attribute`

Display all assets identified by a given class attribute.

## Transfer

`transfer`

Transfer an asset to a given owner.
Throws error if `InstanceNotFound`

## Mint NFT

`mint_nft`

`mint_into` a given wallet and `set_typed_attributes.
Throws error if `NFTCount` returns invalid values