# Assets 

The Asset's pallet provides implementations for common currency traits and functionality for handling transfers and minting.
E.g. from [orml](https://docs.rs/orml-traits) or [frame_support](https://github.com/paritytech/substrate/tree/master/frame/support)

## Overview

The Asset's pallet provides functions for:
- Transferring balances of native and other assets between accounts.
- Mint and burn assets decided by governance on a per-asset basis.
- Crediting and debiting of created asset balances.
- By design, similar to [orml currencies](https://docs.rs/orml-currencies/latest/orml_currencies/)
and [Substrate assets](https://github.com/paritytech/substrate/tree/master/frame/assets),
the asset's governance registry origin checks required authorization for function calls.


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

### Workflows

#### Transfers

The transfer functions provided follow generic functionality with some exceptions.
- Generic; transfer `amount` of `asset` from `origin` to `dest`
- Native; transfer `native asset`, avoids asset lookup and is cheaper
 
- Force; root access transfer 
- Force Native; root access transfer of `native asset`

- All; transfer `all free balance` of `asset` from `origin` to `dest`
- All Native; transfer `all free balance` of `native asset` 


#### Minting, Burning & Governance

Minting can be initialized one of three ways:
1. by simply calling the `mint_into` function
2. `mint_initialize` which is intended for creating wrapped assets with no associated project
3. `mint_initialize_with_governance` to use the democracy pallet to mint further assets

When minting, we ensure that the origin is either admin or governance.
With governance, if the `governance_origin` is set to an owned account we can use signed transactions to keep minting.
The `governance_origin` can be any `origin` registered in the `GovernanceRegistry` including but not limited to:
- a collective
- a single user
- sudo
- multi signature

Using the functionality to `burn_from` we can burn an `amount` of `asset_id` from `dest` account.
