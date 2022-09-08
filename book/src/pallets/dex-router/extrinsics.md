<!-- AUTOMATICALLY GENERATED -->
<!-- Generated at 2022-09-05T18:35:35.10671Z -->

# Dex Router Pallet Extrinsics

## Update Route

[`update_route`](https://dali.devnets.composablefinance.ninja/doc/pallet_dex_router/pallet/enum.Call.html#variant.update_route)

Create, update or remove route.
On successful emits one of `RouteAdded`, `RouteUpdated` or `RouteDeleted`.

## Exchange

[`exchange`](https://dali.devnets.composablefinance.ninja/doc/pallet_dex_router/pallet/enum.Call.html#variant.exchange)

Exchange `amount` of quote asset for `asset_pair` via route found in router.
On successful underlying DEX pallets will emit appropriate event

## Sell

[`sell`](https://dali.devnets.composablefinance.ninja/doc/pallet_dex_router/pallet/enum.Call.html#variant.sell)

Sell `amount` of quote asset for `asset_pair` via route found in router.
On successful underlying DEX pallets will emit appropriate event.

## Buy

[`buy`](https://dali.devnets.composablefinance.ninja/doc/pallet_dex_router/pallet/enum.Call.html#variant.buy)

Buy `amount` of quote asset for `asset_pair` via route found in router.
On successful underlying DEX pallets will emit appropriate event.

## Add Liquidity

[`add_liquidity`](https://dali.devnets.composablefinance.ninja/doc/pallet_dex_router/pallet/enum.Call.html#variant.add_liquidity)

Add liquidity to the underlying pablo pool.
Works only for single pool route.

## Remove Liquidity

[`remove_liquidity`](https://dali.devnets.composablefinance.ninja/doc/pallet_dex_router/pallet/enum.Call.html#variant.remove_liquidity)

Remove liquidity from the underlying pablo pool.
Works only for single pool route.
