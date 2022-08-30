# Overview

Runs transfers from some Composable based parachain to Composable parachain. And other parachains integrations.

We do not use direct XCM messages as these are alfa quality.
So all messages go via Relay. Using Direct XCM messaging open channels amid chains with no hops.

We do not use `Teleport` messages as it is considered unsafe.

## Flow

Each XCMP exchange consists of two phases, setup of connection and transfer.

### Setup

- Communicating parachains pair should be added to Relay
- Each parachain must add other parachain into `ParachainSystem` to allow requests from another chain
- Each parachain setups execution prices and filters to secure XCMP messaging
- Each parachain must add mapping for currency it wants to send to other parachain
- Each parachain must deposit to  Relayer

### Transfer currency

Amounts are defined as next:
```rust
// next tells 1 networks up (jump to relay, find chain with ID, than encode para native asset),
let asset_id = AssetId::Concrete(MultiLocation::new(1, X2(Parachain(PICASSO_PARA_ID), GeneralKey(СurrencyId::PICA.encode())));
// here we encode amount of 42 tokens to be manipulated
let amount_and_asset_id = MultiAsset{ fun : Fungible(42), id: asset_id};
```

Transfer currency is based on sending some named messages interpreted on each chain, but always ends with `dispatch` calls on the target chain.  It is possible to send a message and ask for a callback response about success/fail operation, but that happens not in the same block. For selling out things on DEX, will add `Transact` instruction to appreciate pallet.