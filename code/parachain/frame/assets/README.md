# Assets 

The Asset's pallet provides implementations for common currency traits and functionality for handling transfers and minting.
E.g. from [orml](https://docs.rs/orml-traits) or [frame_support](https://github.com/paritytech/substrate/tree/master/frame/support)

## Overview

The Asset's pallet provides functions for:
- Transferring balances of native and other assets between accounts.
- Mint and burn assets decided by governance on a per-asset basis.
- Crediting and debiting of created asset balances.
- By design, similar to [orml currencies](https://docs.rs/orml-currencies/latest/orml_currencies/)
and [Substrate assets](https://github.com/paritytech/substrate/tree/master/frame/assets).

### Implementations

The Assets' pallet provides implementations for the following traits:
- [Currency](https://github.com/paritytech/substrate/blob/master/frame/support/src/traits/tokens/currency.rs):
Functions for dealing with a fungible asset's system.
- [ReservableCurrency](https://github.com/paritytech/substrate/blob/master/frame/support/src/traits/tokens/currency/reservable.rs):
Functions for dealing with assets that an account can reserve.
- [MultiCurrency](https://docs.rs/orml-traits/latest/orml_traits/currency/trait.MultiCurrency.html):
Abstraction over a fungible multi-currency system.
- [MultiLockableCurrency](https://docs.rs/orml-traits/latest/orml_traits/currency/trait.MultiLockableCurrency.html):
A fungible multi-currency system whose accounts can have liquidity restrictions.
- [MultiReservableCurrency](https://docs.rs/orml-traits/latest/orml_traits/currency/trait.MultiReservableCurrency.html):
A fungible multi-currency system where a user can reserve funds.
- `frame_support::traits::tokens::fungibles::{MutateHold, *}`


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