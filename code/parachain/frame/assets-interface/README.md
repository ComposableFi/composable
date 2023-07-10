# Overview

Interface to create Permissionless assets. Supports CosmWasm assets by adding CosmWasm related extrinsics and storages.

ED for permissionless assets is 1, ratio is None. These assets cant be used as payment until ratio is changed from None.

After asset creation, its information can be edited by the creator of an asset and governance.

AssetCreationFee is set by the governance, until it is set assets cant be created. Account creating a new asset will be charged this amount in PICA. 

Admins of CosmWasm asset can mint and burn such an asset if created via register_asset with is_cosmwasm parameter true extrinsic. 