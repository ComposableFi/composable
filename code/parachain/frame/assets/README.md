# Assets 

Unifies native(runtime base) currency and multi(other minted) tokens under same interface and implements all useful traits to fit into other pallets.  

The `asset` pallet provides implementations for common currency traits and functionality for handling transfers and minting.
E.g. from [orml](https://docs.rs/orml-traits) or [frame_support](https://github.com/paritytech/substrate/tree/master/frame/support)

By design, similar to [orml currencies](https://docs.rs/orml-currencies/latest/orml_currencies/)
and [Substrate assets](https://github.com/paritytech/substrate/tree/master/frame/assets).


### Workflows

#### Transfers

The transfer functions provided follow generic functionality with some exceptions.
- Generic; transfer `amount` of `asset` from `origin` to `dest`
- Native; transfer `native asset`, avoids asset lookup and is cheaper
 
- Force; root access transfer 
- Force Native; root access transfer of `native asset`

- All; transfer `all free balance` of `asset` from `origin` to `dest`
- All Native; transfer `all free balance` of `native asset` 

#### RPCs
`assets_listAssets`

This RPC will return a list of all the registered assets, including
both local and foreign ones.

An example response will look like this:

```json
[
    // Local asset
    {
        "name": [80, 73, 67, 65],
        "id": 1,
        "decimals": 12,
        "foreign_id": null // foreign_id will be null for local
        // assets, unless the XCM location is know and hardcoded
    },
    // Local asset with a known location
    {
        "name": [75, 83, 77],
        "id": 4,
        "decimals": 12,
        "foreign_id": { // location is hardcoded for KSM
            "parents": 0,
            "interior": "Here"
        }
    },
    // Foreign asset
    {
        "name": null, // name will be null for foreign assets
        "id": 12884901886,
        "decimals": 6,
        "foreign_id": {
            "parents": 8,
            "interior": "Here"
        }
    },
    // Foreign asset
    {
        "name": null,
        "id": 12884901887,
        "decimals": 10,
        "foreign_id": {
            "parents": 123,
            "interior": {
                "X1": {
                    "Parachain": 123
                }
            }
        }
    }
]
```