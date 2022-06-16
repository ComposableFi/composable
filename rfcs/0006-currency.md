
# [DRAFT]

This documents overview what currency is, whats is properties are and what API should we have to run various scenarios safely and efficiently.

It mostly considers technica and safety propeties of currency, not bussiness part of it.

Governance of currencies is mostly out of scope of this document too.

## What is currency?

Identity of currency is positive integer. Any positive integer may be currency id, but not all integers are currency ids. That is `MayBeCurrencyId`.

Once id was recoreded into ledger it never can be removed. So it can be restricted and disabled in some form later (`soft deleted`).

Given we mapped `MayBeCurrencyId` into `CurrencyId` we can think of next infromation about it:

- `Amount`. Given id, we can ask for total supply of currency in consensus of ledger. Amount of 10 of currenc is equal to amount of 10 of same currency - it is fungible.
- `Decimals`. Currency may have unit and minimal amount. Unit usually 10 to some power like 6 or 9. Unit usually is priceable and comprehedible by people.  While minimal amount can be used to operat micro transactions and helf to minimize rounding errors. Ledgers operate only in decimal currencies. Zero decimals is viable currency too. Mostly pallets operat in decimals oblivion manner. 
- `Symbol`. Currency may have human readable symbol. Like `XBTC`. This is list is target for governance to prevent spam and fishing.
- `Native Currency`. Currency which is used to pay for operation on platforms. There could be several layers of native currency in single consensus. Native currency of consensus and native currency of protocol built on top.

Above properies apply to local currencies, so to make econimics operate, need to integrate external sources of currencies to. Here we obtain more propertis.

- `Remote Currency Id`. We can have local currency id. Using some forms of cross consesnsus mechanism (like relayers) we can `transfer` currency from one chain to other. So currency may have `Chain Id` from which it came along with id it has on that chain. 
- `Remote Currency Decimals`. When transfer happens, we should know what  given `Amount` transfered means for that currency on our local netowrk. We need to know what are inimal amounts and decimals currency has remotely. This works off chain sources too, like Oracles. This may lead to precision loss and loss of currency too. In worth case transfer back and forth will print money. Example, remotely BTC can have 8 decimals, while locally we can have 12.
- `Trusted Currency`. If there is surce of currency, like Ethereum. And there is trusted consensus (parachain) which can transfer `Etherium` currency into Polkadot relay chains ecosystem. Than that trusted consesus can be used to transfer remote currency to local with 1 to 1 correspondance. So lack of reverse of trust is open question. Process can be improved by setting up oracles which monitor burn proves on each chain. In theory burns and proves could be manual to allow manual.
- `Mapped currency`.  Given possible lack of trust and difference of decimals, we come up the need for market to decide how well these are aligned. So if remote yBTC has local id of 42 and remote id of (200, 13) with 8 decimals and local xBTC has id of 1 and 12 decimals, than local DEX pool ratio of 10_000 should be created. Large enough pools are quantive trust. Automatic mapping and User Interface can help to handle.
- `Remote Symbols`. 

All currencies are derived, so some currencies can be derived in local consensus. So:
- `One to One` when we create new currency, and it is always one to one to some other currency.
- `Dillution`. If currency was direcly derived from protocol in local consensus than can tell exact ration of one amount needed to swap for other amount. With time currency amounts can change (locked, unlocked, minted), which will changed dillution factor. So we cannot tell nothing about external currencies dillution factors, only Oracles can tell. In this case we can observer whole chain of of tokens as they wrap each other and dillute. Process is not nessecary bidirectional.
- `Local Simple Dillution` - when for each amount there is mint of other amount, may be allowing different reserver/hold propreties. May eventually be not one to one as other can be burnt. But because of can witness total supply, can say ratio. So what to do if curreny transfered from and to system?

Also each protocol can have non direcly expressable propeties:
- `Inflation`. Currency supply may be limited, frozen, minting speed depends on time or some activity, time locked. 

## Currency is valid/exists

Having API which assets `MayBeCurrencyId` and returns boolean separately will waste resources. 

Any API accepting MayBeCurrencyId does some form of check. 

Example, if currency does not exists, it cannot be locked, reserverd, transfered because there is 0 of it.

If there will be need to API like that, API which transforms `MayBeCurrencyId` to `CurrencyId` can be created. 
So rest code if exstrinsic can operate safely without any checks.

## Assets registry

What assets registry should proved in general, with link to our assets/assters-registy and registy-govenrancne pallets.

## XCM and other cross consensus protocols

When there are assets in one consensus, how they can be transfered to other consensus?

### Reserver Transfer

Hi there, Parity suggested(at stack exchange) to build your own security protocol on top
Simple one:
Parachain1 (P1) mints A
P1 mints B for A (1-1 map)
P1 sends messages to P2 with B
B consults it registry and mints B' on it.
So P2 can send message to transfer  B' to P1.
P1 will burn burn B and mint A(can do lock)
KSM over relay seems should do that (need to find exact code) or for different tokens on Statemine. No sure about acala - how they settle that.
Can operate on full trust too, like if P2 or P3 sends A token it promised to burn and did it, and we do not need to map it on P1.
I guess schemas like Mosaic relayed  possible too.
7 replies

Karel  4 days ago
So downside of this is compatibility with other bridging solutions. What happens when we do
Acala -- (XCM) --> Picasso -- (IBC) --> Osmosis -- (IBC) --> Astar -- (XCM) --> Acala?

Karel  4 days ago
I think fundamentally that model is not scalable

Karel  4 days ago
Each of these hops would require a consult with Acala.
:heavy_check_mark:
1


Karel  4 days ago
It is also not how clearing actually works

Dzmitry Lahoda  2 days ago
Hi, I am online from today.
XCM is overall agnostics from how we do the trust of para to para. But not sure if default ORML traits (xtokens) handle that agnostics.
Yes, by current design any transfer/swap should go over Reserve (trusted chain) who accounts for balances. It is most secured way I guess.
So we can design different shared security scheme.
Or build on top of existing security scheme. Like UI may LIE (need bunch of RPC endopints) that KSM on Kusama is same as KSM on Acala is same as KSM on Picasso.
And do proper transfer beneath.
That process can be automated via dexes. So transfer will be weight fee + dex fee. But for users it will look like as single chain operation.

Dzmitry Lahoda  2 days ago
 It is also not how clearing actually works
clearing?

Dzmitry Lahoda  2 days ago
I can do research and documenting on various bridge protocols in terms of what security/(de)centralizatoion they provide so can choose proper for uses case.


## API considerations

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
 - should allow to split currency into some ranges defined during genesis 

 Given above we stick with possibly wrong asset id passed into API.

 ## Almos working rust code
 ```rust, ignore
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

TODO: add mention about Euler feature to govern currencies by risks categories

macor