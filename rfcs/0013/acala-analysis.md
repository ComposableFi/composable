### Acala

Acala has four asset classifications:

* Foreign
* Stable
* Erc20
* Native

Acala has separate registration processes for each of these within their Asset 
Registry pallet. Of interest to is are the foreign and native asset registration
process.

#### Foreign Assets

Foreign assets are registered with a `MultiLocation` and `AssetMetadata`. The 
`MultiLocation` then mapped to a `ForeignAssetId` that is determined by a nonce. 
A mapping of `ForeignAssetId` to `MultiLocation` is also created so that either 
can be used to look up the other. The provided metadata provides the assets 
name, symbol, decimals, and minimum balance. The Metadata is indexed by the 
`ForeignAssetId`.

Foreign assets can then be updated by their `ForeignAssetId`, in which case
both the assets `MultiLocation` and `AssetMetadata` can be updated.

#### Native Assets

Native assets are registered with a `CurrencyId` and `AssetMetadata`. The 
`CurrencyId` maps to the `AssetMetadata` and this mapping is used to ensure that 
the asset does not already exist.

Native assets can then be updated by their `CurrencyId`, in which case
the assets `AssetMetadata` can be updated.

#### General Notes

* The `AssetMetadatas` storage map is used for all types and is quarried by the 
`AssetIds` `enum`.

* The `AssetIds` type is also used for routing regular currency functions (i.e. 
transfer, withdraw, deposit).

* For non-erc20 assets, Acala uses 'orml-tokens' for currency functions

#### Data Structures & Types

Note: `MultiLocation` is supplied by XCM.

```rust
pub type ForeignAssetId = u16;
```

```rust
pub struct AssetMetadata<Balance> {
  pub name: Vec<u8>,
	pub symbol: Vec<u8>,
	pub decimals: u8,
	pub minimal_balance: Balance,
}
```

```rust
pub enum AssetIds {
	Erc20(EvmAddress),
	StableAssetId(StableAssetPoolId),
	ForeignAssetId(ForeignAssetId),
	NativeAssetId(CurrencyId),
}
```

```rust
pub enum CurrencyId {
	Token(TokenSymbol),
	DexShare(DexShare, DexShare),
	Erc20(EvmAddress),
	StableAssetPoolToken(StableAssetPoolId),
	LiquidCrowdloan(Lease),
	ForeignAsset(ForeignAssetId),
}
```
