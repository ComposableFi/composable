<!-- AUTOMATICALLY GENERATED -->
<!-- Generated at 2022-09-05T18:35:35.080611Z -->

# Assets Pallet Extrinsics

## Transfer

[`transfer`](https://dali.devnets.composablefinance.ninja/doc/pallet_assets/pallet/enum.Call.html#variant.transfer)

Transfer `amount` of `asset` from `origin` to `dest`.

### Errors

* When `origin` is not signed.
* If the account has insufficient free balance to make the transfer, or if `keep_alive`
  cannot be respected.
* If the `dest` cannot be looked up.

## Transfer Native

[`transfer_native`](https://dali.devnets.composablefinance.ninja/doc/pallet_assets/pallet/enum.Call.html#variant.transfer_native)

Transfer `amount` of the native asset from `origin` to `dest`. This is slightly
cheaper to call, as it avoids an asset lookup.

### Errors

* When `origin` is not signed.
* If the account has insufficient free balance to make the transfer, or if `keep_alive`
  cannot be respected.
* If the `dest` cannot be looked up.

## Force Transfer

[`force_transfer`](https://dali.devnets.composablefinance.ninja/doc/pallet_assets/pallet/enum.Call.html#variant.force_transfer)

Transfer `amount` of the `asset` from `origin` to `dest`. This requires root.

### Errors

* When `origin` is not root.
* If the account has insufficient free balance to make the transfer, or if `keep_alive`
  cannot be respected.
* If the `dest` cannot be looked up.

## Force Transfer Native

[`force_transfer_native`](https://dali.devnets.composablefinance.ninja/doc/pallet_assets/pallet/enum.Call.html#variant.force_transfer_native)

Transfer `amount` of the the native asset from `origin` to `dest`. This requires root.

### Errors

* When `origin` is not root.
* If the account has insufficient free balance to make the transfer, or if `keep_alive`
  cannot be respected.
* If the `dest` cannot be looked up.

## Transfer All

[`transfer_all`](https://dali.devnets.composablefinance.ninja/doc/pallet_assets/pallet/enum.Call.html#variant.transfer_all)

Transfer all free balance of the `asset` from `origin` to `dest`.

### Errors

* When `origin` is not signed.
* If the `dest` cannot be looked up.

## Transfer All Native

[`transfer_all_native`](https://dali.devnets.composablefinance.ninja/doc/pallet_assets/pallet/enum.Call.html#variant.transfer_all_native)

Transfer all free balance of the native asset from `origin` to `dest`.

### Errors

* When `origin` is not signed.
* If the `dest` cannot be looked up.

## Mint Initialize

[`mint_initialize`](https://dali.devnets.composablefinance.ninja/doc/pallet_assets/pallet/enum.Call.html#variant.mint_initialize)

Creates a new asset, minting `amount` of funds into the `dest` account. Intended to be
used for creating wrapped assets, not associated with any project.

## Mint Initialize With Governance

[`mint_initialize_with_governance`](https://dali.devnets.composablefinance.ninja/doc/pallet_assets/pallet/enum.Call.html#variant.mint_initialize_with_governance)

Creates a new asset, minting `amount` of funds into the `dest` account. The `dest`
account can use the democracy pallet to mint further assets, or if the governance_origin
is set to an owned account, using signed transactions. In general the
`governance_origin` should be generated from the pallet id.

## Mint Into

[`mint_into`](https://dali.devnets.composablefinance.ninja/doc/pallet_assets/pallet/enum.Call.html#variant.mint_into)

Mints `amount` of `asset_id` into the `dest` account.

## Burn From

[`burn_from`](https://dali.devnets.composablefinance.ninja/doc/pallet_assets/pallet/enum.Call.html#variant.burn_from)

Burns `amount` of `asset_id` into the `dest` account.
