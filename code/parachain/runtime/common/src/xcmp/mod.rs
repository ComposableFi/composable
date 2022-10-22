//! proposed shared XCM setup parameters and impl
use crate::{AccountId, Balance};
use codec::Decode;
use composable_traits::{
	oracle::MinimalOracle,
	xcm::assets::{RemoteAssetRegistryInspect, XcmAssetLocation},
};
use frame_support::{
	dispatch::Weight,
	ensure, log, parameter_types,
	traits::{Contains, Get},
	weights::{WeightToFee, WeightToFeePolynomial},
};
use num_traits::Zero;
use orml_traits::location::{AbsoluteReserveProvider, Reserve};
use polkadot_primitives::v2::Id;
use primitives::currency::{CurrencyId, WellKnownCurrency};
use sp_runtime::traits::Convert;
use sp_std::marker::PhantomData;
use xcm::{latest::MultiAsset, prelude::*};
use xcm_builder::*;
use xcm_executor::{
	traits::{FilterAssetLocation, ShouldExecute, WeightTrader},
	*,
};

parameter_types! {
	// similar to what Acala/Hydra has
	pub const BaseXcmWeight: Weight = 100_000_000;
	pub const XcmMaxAssetsForTransfer: usize = 2;
}

/// this is debug struct implementing as many XCMP interfaces as possible
/// it just dumps content, no modification.
/// returns default expected
pub struct XcmpDebug;

impl ShouldExecute for XcmpDebug {
	fn should_execute<Call>(
		origin: &MultiLocation,
		message: &mut Xcm<Call>,
		max_weight: Weight,
		weight_credit: &mut Weight,
	) -> Result<(), ()> {
		log::trace!(target: "xcmp::should_execute", "{:?} {:?} {:?} {:?}", origin, message, max_weight, weight_credit);
		Err(())
	}
}

/// is used to represent this chain in various APIS
pub struct ThisChain<T>(PhantomData<T>);

impl<T: Get<Id>> ThisChain<T> {
	pub const SELF_RECURSIVE: MultiLocation = MultiLocation { parents: 0, interior: Here };
	pub fn self_parent() -> MultiLocation {
		MultiLocation { parents: 1, interior: X1(Parachain(T::get().into())) }
	}
}

impl<T: Get<Id>> Contains<MultiLocation> for ThisChain<T> {
	fn contains(origin: &MultiLocation) -> bool {
		origin == &Self::SELF_RECURSIVE || origin == &Self::self_parent()
	}
}

/// NOTE: there could be payments taken on other side, so cannot rely on this to work end to end
pub struct DebugAllowUnpaidExecutionFrom<T>(PhantomData<T>);
impl<T: Contains<MultiLocation>> ShouldExecute for DebugAllowUnpaidExecutionFrom<T> {
	fn should_execute<Call>(
		origin: &MultiLocation,
		_message: &mut Xcm<Call>,
		_max_weight: Weight,
		_weight_credit: &mut Weight,
	) -> Result<(), ()> {
		log::trace!(
			target: "xcm::barriers",
			"AllowUnpaidExecutionFrom origin: {:?}, message: {:?}, max_weight: {:?}, weight_credit: {:?}, contains: {:?}",
			origin, _message, _max_weight, _weight_credit, T::contains(origin),
		);
		ensure!(T::contains(origin), ());
		Ok(())
	}
}

pub struct TransactionFeePoolTrader<
	AssetConverter,
	PriceConverter,
	Treasury: TakeRevenue,
	WeightToFee,
> {
	_marker: PhantomData<(AssetConverter, PriceConverter, Treasury, WeightToFee)>,
	fee: Balance,
	price: Balance,
	asset_location: Option<MultiLocation>,
}

impl<
		AssetConverter: Convert<MultiLocation, Option<CurrencyId>>,
		PriceConverter: MinimalOracle<AssetId = CurrencyId, Balance = Balance>,
		Treasury: TakeRevenue,
		WeightToFeeConverter: WeightToFeePolynomial<Balance = Balance> + WeightToFee<Balance = Balance>,
	> TransactionFeePoolTrader<AssetConverter, PriceConverter, Treasury, WeightToFeeConverter>
{
	pub fn weight_to_asset(
		weight: Weight,
		asset_id: CurrencyId,
	) -> Result<(Balance, Balance), XcmError> {
		let fee = WeightToFeeConverter::weight_to_fee(&weight);
		log::trace!(target : "xcmp::buy_weight", "required payment in native token is: {:?}", fee );
		let price =
			PriceConverter::get_price_inverse(asset_id, fee).map_err(|_| XcmError::TooExpensive)?;
		log::trace!(target : "xcmp::buy_weight", "amount of priceable token to pay fee {:?}", price );
		return Ok((fee, price))
	}
}
impl<
		AssetConverter: Convert<MultiLocation, Option<CurrencyId>>,
		PriceConverter: MinimalOracle<AssetId = CurrencyId, Balance = Balance>,
		Treasury: TakeRevenue,
		WeightToFeeConverter: WeightToFeePolynomial<Balance = Balance> + WeightToFee<Balance = Balance>,
	> WeightTrader
	for TransactionFeePoolTrader<AssetConverter, PriceConverter, Treasury, WeightToFeeConverter>
{
	fn new() -> Self {
		Self {
			_marker:
				PhantomData::<(AssetConverter, PriceConverter, Treasury, WeightToFeeConverter)>::default(),
			fee: 0,
			price: 0,
			asset_location: None,
		}
	}

	fn buy_weight(&mut self, weight: Weight, payment: Assets) -> Result<Assets, XcmError> {
		if weight.is_zero() {
			return Ok(payment)
		}

		// only support first fungible assets now.
		let xcmp_asset_id = payment
			.fungible
			.iter()
			.next()
			.map_or(Err(XcmError::TooExpensive), |v| Ok(v.0))?;

		if let AssetId::Concrete(ref multi_location) = xcmp_asset_id.clone() {
			if let Some(asset_id) = AssetConverter::convert(multi_location.clone()) {
				let (fee, price) = Self::weight_to_asset(weight, asset_id)?;
				let required =
					MultiAsset { id: xcmp_asset_id.clone(), fun: Fungibility::Fungible(price) };

				log::trace!(target : "xcmp::buy_weight", "required priceable token {:?}; provided payment:{:?} ", required, payment );
				let unused = payment.checked_sub(required).map_err(|_| XcmError::TooExpensive)?;

				self.fee = self.fee.saturating_add(fee);
				self.price = self.price.saturating_add(price);
				self.asset_location = Some(multi_location.clone());
				return Ok(unused)
			}
		}

		log::info!(target : "xcmp::buy_weight", "required {:?}; provided {:?};", weight, payment );
		Err(XcmError::TooExpensive)
	}

	fn refund_weight(&mut self, weight: Weight) -> Option<MultiAsset> {
		if let Some(ref asset_location) = self.asset_location {
			let fee = WeightToFeeConverter::weight_to_fee(&weight);
			let fee = self.fee.min(fee);
			let price = fee.saturating_mul(self.price) / self.fee;
			self.price = self.price.saturating_sub(price);
			self.fee = self.fee.saturating_sub(fee);
			if price > 0 {
				return Some((asset_location.clone(), price).into())
			}
		}

		None
	}
}

pub struct ToTreasury<AssetsConverter, Assets, TreasuryAccount>(
	PhantomData<(AssetsConverter, Assets, TreasuryAccount)>,
);
impl<
		AssetsConverter: Convert<MultiLocation, Option<CurrencyId>>,
		Assets: orml_traits::currency::MultiCurrency<AccountId, CurrencyId = CurrencyId, Balance = Balance>,
		Treasury: Get<AccountId>,
	> TakeRevenue for ToTreasury<AssetsConverter, Assets, Treasury>
{
	fn take_revenue(revenue: MultiAsset) {
		if let MultiAsset { id: Concrete(location), fun: Fungible(amount) } = revenue {
			log::info!(target: "xcmp::take_revenue", "{:?} {:?}", &location, amount);
			if let Some(currency_id) = AssetsConverter::convert(location) {
				let account = &Treasury::get();
				log::info!(target: "xcmp::take_revenue", "{:?} {:?} {:?}", &currency_id, &account, amount);
				match <Assets>::deposit(currency_id, account, amount) {
					Ok(_) => {},
					Err(err) => log::error!(target: "xcmp::take_revenue", "{:?}", err),
				};
			} else {
				log::error!(target: "xcmp::take_revenue", "failed to convert revenue currency");
			}
		}
	}
}

/// is called to convert some account id to account id on other network
/// as of now it is same as in Acala/Hydra
pub struct AccountIdToMultiLocation;
impl Convert<AccountId, MultiLocation> for AccountIdToMultiLocation {
	fn convert(account: AccountId) -> MultiLocation {
		//  considers any other network using globally unique ids
		X1(AccountId32 { network: NetworkId::Any, id: account.into() }).into()
	}
}

/// well know XCMP origin
pub trait XcmpAssets {
	fn remote_to_local(location: MultiLocation) -> Option<CurrencyId>;
	fn local_to_remote(id: CurrencyId, this_para_id: u32) -> Option<MultiLocation>;
}

/// Converts currency to and from local and remote
pub struct CurrencyIdConvert<AssetRegistry, WellKnownCurrency, ThisParaId, WellKnownXcmpAssets>(
	PhantomData<(AssetRegistry, WellKnownCurrency, ThisParaId, WellKnownXcmpAssets)>,
);

/// converts local currency into remote,
/// native currency is built in
impl<
		AssetRegistry: RemoteAssetRegistryInspect<AssetId = CurrencyId, AssetNativeLocation = XcmAssetLocation>,
		WellKnown: WellKnownCurrency,
		ThisParaId: Get<Id>,
		WellKnownXcmpAssets: XcmpAssets,
	> sp_runtime::traits::Convert<CurrencyId, Option<MultiLocation>>
	for CurrencyIdConvert<AssetRegistry, WellKnown, ThisParaId, WellKnownXcmpAssets>
{
	fn convert(id: CurrencyId) -> Option<MultiLocation> {
		if let Some(location) = WellKnownXcmpAssets::local_to_remote(id, ThisParaId::get().into()) {
			Some(location)
		} else if let Some(location) = AssetRegistry::asset_to_remote(id).map(|x| x.location.into())
		{
			Some(location)
		} else {
			log::trace!(
				target: "xcmp:convert",
				"mapping for {:?} on {:?} parachain not found",
				id,
				ThisParaId::get()
			);
			None
		}
	}
}

// must be a non-associated const to allow for pattern matching
pub const RELAY_LOCATION: MultiLocation = MultiLocation { parents: 1, interior: Here };

/// converts remote asset to local
/// 1. if remote is origin without key(some identifiers), than it is native token
/// 2. if origin is parent of this consensus, than this is relay
/// 2. if origin is this consensus, than it is this native token
/// 3. if origin is some well know chain and key(asset id) is exactly same as binary value on remote
/// chain, that we map to local currency 4. if origin is mapped by sender to include our mapped id
/// into our chain, than we also map that
///
/// so:
/// 1. in some cases origin leads to asset id
/// 2. in some well know cases remote asset id is statically typed into here (so it is okay to send
/// their id to us) 3. and in other cases they must map on us, and than send our id to here
impl<
		AssetsRegistry: RemoteAssetRegistryInspect<AssetId = CurrencyId, AssetNativeLocation = XcmAssetLocation>,
		WellKnown: WellKnownCurrency,
		ThisParaId: Get<Id>,
		WellKnownXcmpAssets: XcmpAssets,
	> Convert<MultiLocation, Option<CurrencyId>>
	for CurrencyIdConvert<AssetsRegistry, WellKnown, ThisParaId, WellKnownXcmpAssets>
{
	fn convert(location: MultiLocation) -> Option<CurrencyId> {
		log::trace!(target: "xcmp::convert", "converting {:?} on {:?}", &location, ThisParaId::get());
		match location {
			MultiLocation { parents, interior: X2(Parachain(id), GeneralKey(key)) }
				if parents == 1 && Id::from(id) == ThisParaId::get() =>
				CurrencyId::decode(&mut &key[..]).ok(),
			MultiLocation { parents: 0, interior: X1(GeneralKey(key)) } =>
				CurrencyId::decode(&mut &key[..]).ok(),
			RELAY_LOCATION => Some(CurrencyId::RELAY_NATIVE),
			ThisChain::<ThisParaId>::SELF_RECURSIVE => Some(CurrencyId::NATIVE),
			_ =>
				if let Some(currency_id) = WellKnownXcmpAssets::remote_to_local(location.clone()) {
					Some(currency_id)
				} else {
					log::trace!(target: "xcmp", "using assets registry for {:?}", location);
					let result = AssetsRegistry::location_to_asset(XcmAssetLocation(location))
						.map(Into::into);
					if let Some(result) = result {
						log::trace!(target: "xcmp", "mapped remote to {:?} local", result);
					} else {
						log::trace!(target: "xcmp", "failed converting currency");
					}

					result
				},
		}
	}
}

/// covert remote to local, usually when receiving transfer
impl<
		T: RemoteAssetRegistryInspect<AssetId = CurrencyId, AssetNativeLocation = XcmAssetLocation>,
		WellKnown: WellKnownCurrency,
		ThisParaId: Get<Id>,
		WellKnownXcmpAssets: XcmpAssets,
	> Convert<MultiAsset, Option<CurrencyId>>
	for CurrencyIdConvert<T, WellKnown, ThisParaId, WellKnownXcmpAssets>
{
	fn convert(asset: MultiAsset) -> Option<CurrencyId> {
		log::trace!(target: "xcmp", "converting {:?}", &asset);
		if let MultiAsset { id: Concrete(location), .. } = asset {
			<Self as Convert<MultiLocation, Option<CurrencyId>>>::convert(location)
		} else {
			log::error!(target: "xcmp", "failed to find remote asset");
			None
		}
	}
}

pub struct DebugMultiNativeAsset;
impl FilterAssetLocation for DebugMultiNativeAsset {
	fn filter_asset_location(asset: &MultiAsset, origin: &MultiLocation) -> bool {
		log::trace!(
			target: "xcmp::filter_asset_location",
			"asset: {:?}; origin: {:?}; reserve: {:?};",
			&asset,
			&origin,
			AbsoluteReserveProvider::reserve(&asset.clone()),
		);
		false
	}
}

impl<X, Y, Treasury: TakeRevenue, Z> Drop for TransactionFeePoolTrader<X, Y, Treasury, Z> {
	fn drop(&mut self) {
		log::info!(target : "xcmp::take_revenue", "{:?} {:?}", &self.asset_location, self.fee);
		if let Some(asset) = self.asset_location.take() {
			if self.price > Balance::zero() {
				Treasury::take_revenue((asset, self.price).into());
			}
		}
	}
}
