<!-- AUTOMATICALLY GENERATED -->
<!-- Generated at 2022-06-25T22:31:58.281456585Z -->

# Assets Registry Pallet Extrinsics

## Register Asset

[`register_asset`](https://dali.devnets.composablefinance.ninja/doc/pallet_assets_registry/pallet/enum.Call.html#variant.register_asset)

creates asset using `CurrencyFactory`,
raises `AssetRegistered` event

## Update Asset

[`update_asset`](https://dali.devnets.composablefinance.ninja/doc/pallet_assets_registry/pallet/enum.Call.html#variant.update_asset)

Given well existing asset, update its remote information.
Use with caution as it allow reroute assets location.

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
