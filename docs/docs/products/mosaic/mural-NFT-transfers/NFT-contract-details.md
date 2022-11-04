# NFT Contract Details

## Functions

```markdown
function transferERC721ToLayer( 
    address sourceNFTAddress,
    uint256 sourceNFTId,
    address destinationAddress,
    uint256 destinationNetworkID,
    uint256 transferDelay,
    address feeToken
)
```

This function emits `TransferInitiated` with `isRelease == true` if the NFT is to be released on the destination 
network. This is a special case when the original network of the NFT is equal to the destination network. On receiving 
this event the relayer has to call the `releaseSeal` method on the destination layer.

This function emits `TransferInitiated` with `isRelease == false` when the NFT is being transferred to a destination 
network other than the original network of the NFT. `TransferInitiated` event contains the information about the 
original NFT which is needed in the `summonNFT` method. On receiving this event the relayer has to call the `summonNFT` 
method on the destination layer.


```markdown
function releaseSeal(
    address nftOwner,
    address nftContract,
    uint256 nftId,
    bytes32 id,
    bool isFailure
)
```


This method is called in two cases:



1. To release the original NFT on the original network. The boolean `isFailure` is false.
2. To release the source NFT on the source network in case of a failed transfer. The boolean `isFailure` is true.


```markdown
summonNFT(
    string memory nftUri,
    address destinationAddress,
    address originalNftAddress,
    uint256 originalNetworkID,
    uint256 originalNftId,
    bytes32 id
)
```


This method is called to create/transfer a MosaicNFT on the destination network.



## NFT Transfer Flow

### Transfer to destination layer

* User calls `transferERC721ToLayer` on source layer -> `TransferInitiated` event with `isRelease == false`
* Relayer reads the event on source layer and calls `summonNFT` on destination layer
* MosaicNFT is minted/transferred on destination layer


### Transfer back to original layer

* User calls `transferERC721ToLayer` on the original layer with minted NFT -> `TransferInitiated` event with 
  `isRelease == true`
* Relayer reads the event and calls `releaseSeal` on original layer with `isFailure == false`
* Original NFT is transferred back to the user


### Failed transfer to destination layer

* User calls `transferERC721ToLayer` on source layer -> `TransferInitiated` event
* Relayer fails to transfer the NFT to destination layer
* Relayer calls `releaseSeal` on the source with `isFailure == true`


---


## NFT Fees

Fees to transfer your NFT are charged the following approximate fees, contingent on gas costs:

* All layers to L1 will be charged at around $160 USD
* All layers to Arbitrum will be charged at around $10 USD


---


## Supported Layers

The supported layers for the Mosaic NFT transferal system initially include:

* Mainnet
* Polygon
* Arbitrum
* Moonriver
