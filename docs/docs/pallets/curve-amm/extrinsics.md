<!-- AUTOMATICALLY GENERATED -->
<!-- Generated at 2022-04-22T18:59:06.878245736Z -->

# Curve Amm Pallet Extrinsics

## Create

[`create`](https://dali.devnets.composablefinance.ninja/doc/pallet_curve_amm/pallet/enum.Call.html#variant.create)

Create a new pool.

Emits `PoolCreated` event when successful.

## Buy

[`buy`](https://dali.devnets.composablefinance.ninja/doc/pallet_curve_amm/pallet/enum.Call.html#variant.buy)

Execute a buy order on pool.

Emits `Swapped` event when successful.

## Sell

[`sell`](https://dali.devnets.composablefinance.ninja/doc/pallet_curve_amm/pallet/enum.Call.html#variant.sell)

Execute a sell order on pool.

Emits `Swapped` event when successful.

## Swap

[`swap`](https://dali.devnets.composablefinance.ninja/doc/pallet_curve_amm/pallet/enum.Call.html#variant.swap)

Execute a specific swap operation.

The `quote_amount` is always the quote asset amount (A/B => B), (B/A => A).

Emits `Swapped` event when successful.

## Add Liquidity

[`add_liquidity`](https://dali.devnets.composablefinance.ninja/doc/pallet_curve_amm/pallet/enum.Call.html#variant.add_liquidity)

Add liquidity to a stable-swap pool.

Emits `LiquidityAdded` event when successful.

## Remove Liquidity

[`remove_liquidity`](https://dali.devnets.composablefinance.ninja/doc/pallet_curve_amm/pallet/enum.Call.html#variant.remove_liquidity)

Remove liquidity from stable-swap pool.

Emits `LiquidityRemoved` event when successful.
