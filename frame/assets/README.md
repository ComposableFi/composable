# Assets 

The Assets pallet provides implementations for common currency traits and functionality for handling transfers and minting.
e.g. from [orml](https://docs.rs/orml-traits) or `frame_support`

- `Config`
- `Call`
- `Pallet`

## Overview

The Assets pallet provides functions for:
- Transferring balances of native and other assets between accounts.
- Mint and burn new assets decided on a per-asset basis.
- Crediting and debiting of created asset balances.
- By design, similar to [orml currencies](https://docs.rs/orml-currencies/latest/orml_currencies/)
and [Substrate assets](https://github.com/paritytech/substrate/tree/master/frame/assets),
the asset's governance registry origin checks function calls requiring authorization.


### Implementations

The Assets' pallet provides implementations for the following traits:

- `Currency` (frame_support::traits::Currency):
Functions for dealing with a fungible assets system.
- `ReservableCurrency` (frame_support::traits::ReservableCurrency):
Functions for dealing with assets that an account can reserve.
- `MultiCurrency` (orml_traits::MultiCurrency):
Abstraction over a fungible multi-currency system.
- `MultiLockableCurrency` (orml_traits::MultiLockableCurrency):
A fungible multi-currency system whose accounts can have liquidity restrictions.
- `MultiReservableCurrency` (orml_traits::MultiReservableCurrency):
A fungible multi-currency system where a user can reserve funds.
