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

## Runaway

List of useful changes to do.

Test Assets TX payments 
https://github.com/AcalaNetwork/Acala/commit/88193d6b3f636e483a916a355e1db7a89d38a60b#diff-79521dd3ae35d7e19dff40c49b325850fbad442c1f09d742cf8f03306ef77188

Ensure trapped assets are to claim
https://github.com/AcalaNetwork/Acala/commit/f40e8f9277fe2fabefd4b51d8d2cfd97f088f3b1#diff-4918885dbae3244dd19ee256ec2d575908d8b599007adc761b8651082c4b3288

Add barrier and ED tests

https://github.com/AcalaNetwork/Acala/commit/7a1b02961a9d795d1a62e9ab6e43c5735e244e6f#diff-4918885dbae3244dd19ee256ec2d575908d8b599007adc761b8651082c4b3288R606


Run not only Kusama spec, but polkadot too

https://github.com/AcalaNetwork/Acala/commit/c4f40d1bfba1405c775ba87f57dd17d309290403#diff-9514ad9ceca0c0b988d2614e422ce1366ae94b403f6e1513a47315b7fcb9c21a

Unignore all tests and fix them (broken on some upgrade). 

Make all tests using calculated fees our of types, not hardcoded (can be used in future to build RPC for fee calculator).