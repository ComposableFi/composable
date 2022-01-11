
# [DRAFT]

Currency API.

 Asset id as if it was deserialized, not necessary exists.
 We could check asset id during serde, but that:
 - will make serde setup complicated (need to write and consistently apply static singletons
   to all places with asset id)
 - validate will involve at minimum in memory cache call (in worth case db call) during
   extrinsic invocation
 - will need to disable this during calls when it is really no need for validation (new
   currency mapping)
 - normal path will pay price (validate each time), in instead when fail pays only (like
   trying to transfer non existing asset id)
 - we cannot guarantee existence of asset as it may be removed during transaction (so we
   should make removal exclusive case)

 Given above we stick with possibly wrong asset id passed into API.

 # Assert id pallet design   
 ```ignore
 pub trait MaximalConstGet<T> {
     const VALUE: T;
 }
 /// knows existing local assets and how to map them to simple numbers
 pub trait LocalAssetsRegistry {
    /// asset id which is exist from now in current block
    /// valid does not means usable, it can be subject to deletion or not yet approved to be used
    type AssetId : AssetIdLike + Into<Self::MayBeAssetId>;
    /// just id after serde
    type MayBeAssetId : AssetIdLike + From<Self::AssetId>;
    /// assets which we well know and embedded into `enum`.
    /// maximal of this is smaller than minimal `OtherAssetId`
    /// can always convert to valid asset id
    type WellKnownAssetId : MaximalConstGet<u8> + Into<Self::AssetId> + Into<Self::MayBeAssetId> + Decimals<WellKnownAssetId> + TryFrom<u8>;

    /// Larger than maximal of `WellKnownAssetId` but smaller than minimal `DerivativeAssetId`.
    type OtherAssetId : MinimalConstGet<Self::WellKnownAssetId> + MaximalConstGet<u128>  + Into<Self::AssetId> + Into<Self::MayBeAssetId>;
    /// allows to get next asset id
    /// can consider split out producing assets interface into separate trait
    type NextOtherAssetId = ErrorNext<OtherAssetId>;

    /// locally diluted derivative and liquidity assets.
    /// larger than maximal `OtherAssetId`
    /// `Self::OtherAssetId` may be diluted(derived/wrapped), but only remote.
    type DerivativeAssetId: MinimalConstGet<Self::OtherAssetId> + Into<Self::AssetId>;
    /// may consider split out asset producing trait
    type NextDerivativeAssetId = ErrorNext<Self::DerivativeAssetId>;

    // note: fn to be replaced with Get or traits, just shortcuted here
  
    fn try_from<N:From<MayBeAssetId>>(value : N) -> Result<Self::AssetId, DispatchError>;
    /// one unique native asset id
    fn native() -> Self::WellKnownAssetId;

 }
 /// read remote paths
 /// registering is separate trait
 pub trait RemoteAssetRegistry : LocalAssetsRegistry {
    fn substrate(asset_id: Self::AssetId) -> Self:XcmPath;
    fn remote(asset_id: Self::AssetId, network_id:) -> Self::Path;
 }
 ```
 // NOTE: next is easy to ast macro like 
 ```ignore
 currency!(PICA, 12, milli, micro, 1);
 currency!(SOL, 12, 42);
 // produces enumeration which is into u16, into u128, try from u128, enumerate, get decimals, get maximal possible 
 well_known!(PICA, SOL);
 ```
 well known native currency
 #[derive(
 	Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord, TypeInfo, CompactAs, 
 )]
 #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
 #[repr(transparent)]
 pub struct PICA(u128);
 impl Default for PICA {
     fn default() -> Self {
         Self(PICA::ID.into())
     }
 }
 pub trait WellKnownAssetId
 where
 	Self: Copy,
 {
 	const ID: u16;
 	const DECIMALS: Exponent;
 	fn unit<T: From<u64>>() -> T {
 		T::from(10_u64.pow(Self::DECIMALS))
 	}
 }
 impl PICA {
 	// can make in const expression
 	#[inline(always)]
 	pub fn milli<T: From<u64> + Div<Output = T>>() -> T {
 		Self::unit::<T>() / T::from(1000_u64)
 	}	
 }
 impl WellKnownAssetId for PICA {
 	const ID: u16  = 1; 
 	const DECIMALS : Exponent = 12;
 }
 impl Into<u128> for PICA {
     fn into(self) -> u128 {
         self.0
     }
 }

 - assets-registy
 - xcm and other protocols
 - equal currency (btc from btc or eth from eth)
 - non comparable (btc from hydra)
 - decimals of xcm, our btc with 12 and their btc with 8 decimals transfers
 - symbols