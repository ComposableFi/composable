# Liquidation Interface

### High Level Flow

*   Angular calls pallet-liquidation (on Picasso) to dispatch a liquidation order.
*   pallet-liquidation puts the funds in holding/burns the funds, and dispatches a message to the appropriate liquidation engine based on the assets.
*   The liquidation-engine receives the message, mints the collateral it will definitely liquidate, and must then execute the liquidation and immediately return the amount liquidated (collateral asset amount), and an estimate of the liquidated value (borrow asset amount).
*   Angular observes the returned value, mints the lower-bound of the estimated value (borrow-asset amount), and if there is an amount left to be liquidated (collateral asset), falls back to a different liquidation-engine. Default order will be defined by Lending pool governance.
*   The liquidation-engine sends a final acknowledgement of the total liquidated value (borrow-asset) once the entire process is completed. It then burns the borrow-asset, keeping the collateral on the liquidation-engine chain.
*   Angular burns the collateral funds on hold and mints the difference between the lower-bound and the actual borrow asset value of the collateral.


#### Liquidation Order

The following Rust datatype describes initial message sent by pallet-liquidation. Angular will use ReserveTransferAssets to store the assets-to-be-liquidated in the liquidation engine chain.

```plain
struct LiquidationOrder {
    id: Hash,
    assets: 10 BTC,
    minimum_price: 30k kUSD,
}
```


#### Liquidation Acknowledgment

The liquidation-engine may return `InitialAck`, in cases where it is unable to liquidate the entire `amount_taken` immediately. If it is able to do so, it may omit `InitialAck`


```plain
struct InitialAck {
    id: Hash,
    /// borrow asset amounts
    price_bounds: Range<u128>,
    /// collateral asset amount accepted, can be less than we wanted
    /// so can proceed with next engine
    amount_taken: u128,
}
```


After the engine has liquidated the entire `amount_taken` and is able to provide the true price, it transmits `FinalAck`. It MUST uphold to following:


`FinalAck.final_price >= LiquidationOrder.minimum_price`

`InitialAck.bounds.lower >= LiquidationOrder.minimum_price`

`InitialAck.bounds.lower <= FinalAck.final_price`

`InitialAck.amount_taken == FinalAck.amount_taken`


Or same but with single equation `LiquidationOrder.minimum_price <= InitialAck.bounds.lower <= FinalAck.final_price`


```plain
struct FinalAck {
    id: Hash,
    final_price: u128,
    amount_taken: u128,
}
```
