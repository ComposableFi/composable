### Moonbeam

Moonbeam has two asset classifications: 

* Local
* Foreign

Moonbeam maintains separate instances of Parity's pallet-assets to track each of
these asset classifications. Another pallet, pallet-asset-manager, is used to 
route between the two.

Local asset IDs are determined by the result of hashing a counter that tracks
the number of local assets. While Foreign asset IDs are determined by the assets
XCM Multilocation.

#### Foreign

Local assets are registered with a `ForeignAssetType`, `AssetRegisterarMetadata`, a `Balance` representing the ED, and a `bool` `is_sufficent`.
