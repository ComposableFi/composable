<!-- AUTOMATICALLY GENERATED -->
<!-- Generated at 2022-04-22T18:59:06.84606063Z -->

# Uniswap V2 Pallet Extrinsics

## Create

[`create`](https://dali.devnets.composablefinance.ninja/doc/pallet_uniswap_v2/pallet/enum.Call.html#variant.create)

Create a new pool.

Emits `PoolCreated` even when successful.

## Buy

[`buy`](https://dali.devnets.composablefinance.ninja/doc/pallet_uniswap_v2/pallet/enum.Call.html#variant.buy)

Execute a buy order on a pool.

The `base_amount` always represent the base asset amount (A/B => A).

Emits `Swapped` event when successful.

## Sell

[`sell`](https://dali.devnets.composablefinance.ninja/doc/pallet_uniswap_v2/pallet/enum.Call.html#variant.sell)

Execute a sell order on a pool.

The `base_amount` always represent the base asset amount (A/B => A).

Emits `Swapped` event when successful.

## Swap

[`swap`](https://dali.devnets.composablefinance.ninja/doc/pallet_uniswap_v2/pallet/enum.Call.html#variant.swap)

Execute a specific swap operation.

Buy operation if the pair is the original pool pair (A/B).
Sell operation if the pair is the original pool pair swapped (B/A).

The `quote_amount` is always the quote asset amount (A/B => B), (B/A => A).

Emits `Swapped` event when successful.

## Add Liquidity

[`add_liquidity`](https://dali.devnets.composablefinance.ninja/doc/pallet_uniswap_v2/pallet/enum.Call.html#variant.add_liquidity)

Add liquidity to a constant_product pool.

Emits `LiquidityAdded` event when successful.

## Remove Liquidity

[`remove_liquidity`](https://dali.devnets.composablefinance.ninja/doc/pallet_uniswap_v2/pallet/enum.Call.html#variant.remove_liquidity)

Remove liquidity from constant_product pool.

Emits `LiquidityRemoved` event when successful.
