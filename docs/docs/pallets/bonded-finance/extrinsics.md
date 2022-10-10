<!-- AUTOMATICALLY GENERATED -->
<!-- Generated at 2022-09-05T18:35:35.083031Z -->

# Bonded Finance Pallet Extrinsics

## Offer

[`offer`](https://dali.devnets.composablefinance.ninja/doc/pallet_bonded_finance/pallet/enum.Call.html#variant.offer)

Create a new bond offer. To be `bond` to later.

The dispatch origin for this call must be *Signed* and the sender must have the
appropriate funds to stake the offer.

Allows the issuer to ask for their account to be kept alive using the `keep_alive`
parameter.

Emits a `NewOffer`.

## Bond

[`bond`](https://dali.devnets.composablefinance.ninja/doc/pallet_bonded_finance/pallet/enum.Call.html#variant.bond)

Bond to an offer.

The issuer should provide the number of contracts they are willing to buy.
Once there are no more contracts available on the offer, the `stake` put by the
offer creator is refunded.

The dispatch origin for this call must be *Signed* and the sender must have the
appropriate funds to buy the desired number of contracts.

Allows the issuer to ask for their account to be kept alive using the `keep_alive`
parameter.

Emits a `NewBond`.
Possibly Emits a `OfferCompleted`.

## Cancel

[`cancel`](https://dali.devnets.composablefinance.ninja/doc/pallet_bonded_finance/pallet/enum.Call.html#variant.cancel)

Cancel a running offer.

Blocking further bonds but not cancelling the currently vested rewards. The `stake` put
by the offer creator is refunded.

The dispatch origin for this call must be *Signed* and the sender must be `AdminOrigin`

Emits a `OfferCancelled`.
