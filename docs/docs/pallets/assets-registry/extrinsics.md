<!-- AUTOMATICALLY GENERATED -->
<!-- Generated at 2022-09-05T18:35:35.117745Z -->

# Assets Registry Pallet Extrinsics

## Register Asset

[`register_asset`](https://dali.devnets.composablefinance.ninja/doc/pallet_assets_registry/pallet/enum.Call.html#variant.register_asset)

Creates asset using `CurrencyFactory`.
Raises `AssetRegistered` event

Sets only required fields by `CurrencyFactory`, to upsert metadata use referenced
pallet.

### Parameters:

`ratio` -  allows `bring you own gas` fees.
Set to `None` to prevent payment in this asset, only transferring.
Setting to some will NOT start minting tokens with specified ratio.
Foreign assets will be put into parachain treasury as is.

````python
# if cross chain message wants to pay tx fee with non native token
# then amount of native token would be:
amount_of_native_token = amount_of_foreign_token * ratio
````

Examples:

* One to one conversion is 10^18 integer.

* 10\*10^18 will tell that for 1 foreign asset can `buy` 10 local native.

`decimals` - remote number of decimals on other(remote) chain

`ed` - same meaning as in `CurrencyFactory`

## Update Asset

[`update_asset`](https://dali.devnets.composablefinance.ninja/doc/pallet_assets_registry/pallet/enum.Call.html#variant.update_asset)

Given well existing asset, update its remote information.
Use with caution as it allow reroute assets location.
See `register_asset` for parameters meaning.

## Set Min Fee

[`set_min_fee`](https://dali.devnets.composablefinance.ninja/doc/pallet_assets_registry/pallet/enum.Call.html#variant.set_min_fee)

Minimal amount of asset_id required to send message to other network.
Target network may or may not accept payment.
Assumed this is maintained up to date by technical team.
Mostly UI hint and fail fast solution.
In theory can be updated by parachain sovereign account too.
If None, than it is well known cannot pay with that asset on target_parachain_id.
If Some(0), than price can be anything greater or equal to zero.
If Some(MAX), than actually it forbids transfers.
