//! proposed shared XCM setup parameters and impl
use crate::{
	topology::{self},
	AccountId, Balance,
};
use composable_traits::{
	oracle::MinimalOracle,
	xcm::assets::{RemoteAssetRegistryInspect, XcmAssetLocation},
};
use frame_support::{
	dispatch::Weight,
 	log, parameter_types,
	traits::{Contains, Get},
	weights::{WeightToFee, WeightToFeePolynomial},
	WeakBoundedVec, match_types,
};
use num_traits::{One, Zero};
use orml_traits::location::{AbsoluteReserveProvider, Reserve};
use polkadot_primitives::v2::Id;
use primitives::currency::{CurrencyId, WellKnownCurrency};
use sp_runtime::traits::Convert;
use sp_std::marker::PhantomData;
use xcm::{latest::MultiAsset, prelude::*};
use xcm_builder::*;
use xcm_executor::{
	traits::{FilterAssetLocation, WeightTrader},
	*,
};
use cumulus_primitives_core::ParaId; 

pub const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	

match_types! {
	pub type ParentOrSiblings: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: Here } |
		MultiLocation { parents: 1, interior: X1(_) }
	};
}

parameter_types! {
	pub const BaseXcmWeight: Weight = 100_000_000;
	pub const XcmMaxAssetsForTransfer: usize = 2;
	pub RelayNativeLocation: MultiLocation = MultiLocation::parent();
	pub RelayOrigin: cumulus_pallet_xcm::Origin = cumulus_pallet_xcm::Origin::Relay;
	pub const UnitWeightCost: Weight = 200_000_000;
	pub const MaxInstructions: u32 = 100;
}
pub struct ThisChain<T>(PhantomData<T>);

impl<T: Get<Id>> ThisChain<T> {
	pub fn self_parent() -> MultiLocation {
		topology::this::sibling(T::get().into())
	}
}

impl<T: Get<Id>> Contains<MultiLocation> for ThisChain<T> {
	fn contains(origin: &MultiLocation) -> bool {
		origin == &topology::this::Local::get() || origin == &Self::self_parent()
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
		log::trace!(target : "xcmp::weight_to_asset", "required payment in native token is: {:?}", fee );
		let price =
			PriceConverter::get_price_inverse(asset_id, fee).map_err(|_| XcmError::TooExpensive)?;
		let price = price.max(Balance::one());
		log::trace!(target : "xcmp::weight_to_asset", "amount of priceable token to pay fee {:?}", price );
		Ok((fee, price))
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

pub trait XcmpAssets {
	fn remote_to_local(location: MultiLocation) -> Option<CurrencyId> {
		match location {
			MultiLocation { parents: 1, interior: X2(Parachain(para_id), GeneralKey(key)) } =>
				match (para_id, &key[..]) {
					(topology::karura::ID, topology::karura::KUSD_KEY) => Some(CurrencyId::kUSD),
					_ => None,
				},
			MultiLocation {
				parents: 1,
				interior: X3(Parachain(para_id), PalletInstance(pallet_instance), GeneralIndex(key)),
			} => match (para_id, pallet_instance, key) {
				(
					topology::common_good_assets::ID,
					topology::statemine::ASSETS,
					topology::statemine::USDT,
				) => Some(CurrencyId::USDT),
				_ => None,
			},
			_ => None,
		}
	}

	fn local_to_remote(id: CurrencyId, _this_para_id: u32) -> Option<MultiLocation> {
		match id {
			CurrencyId::NATIVE => Some(topology::this::Local::get()),
			CurrencyId::PBLO => Some(MultiLocation {
				parents: 0,
				interior: X1(GeneralIndex(CurrencyId::PBLO.into())),
			}),
			CurrencyId::RELAY_NATIVE => Some(MultiLocation::parent()),
			CurrencyId::kUSD => Some(MultiLocation {
				parents: 1,
				interior: X2(
					Parachain(topology::karura::ID),
					GeneralKey(WeakBoundedVec::force_from(
						topology::karura::KUSD_KEY.to_vec(),
						None,
					)),
				),
			}),
			CurrencyId::USDT => Some(MultiLocation {
				parents: 1,
				interior: X3(
					Parachain(topology::common_good_assets::ID),
					PalletInstance(topology::common_good_assets::ASSETS),
					GeneralIndex(topology::common_good_assets::USDT),
				),
			}),
			_ => None,
		}
	}
}


/// Converts currency to and from local and remote.
/// Checks compile time and runtime assets mapping.
pub struct CurrencyIdConvert<AssetRegistry, WellKnownCurrency, ThisParaId, WellKnownXcmpAssets>(
	PhantomData<(AssetRegistry, WellKnownCurrency, ThisParaId, WellKnownXcmpAssets)>,
);


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
			topology::relay::LOCATION => Some(CurrencyId::RELAY_NATIVE),
			topology::this::LOCAL => Some(CurrencyId::NATIVE),
			MultiLocation { parents, interior: X2(Parachain(id), GeneralIndex(index)) }
				if parents == 1 && Id::from(id) == ThisParaId::get() =>
				Some(CurrencyId(index)),
			MultiLocation { parents: 0, interior: X1(GeneralIndex(index)) } =>
				Some(CurrencyId(index)),
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
pub struct RelayReserveFromParachain;
impl FilterAssetLocation for RelayReserveFromParachain {
	fn filter_asset_location(asset: &MultiAsset, origin: &MultiLocation) -> bool {
		AbsoluteReserveProvider::reserve(asset) == Some(MultiLocation::parent()) &&
			matches!(origin, MultiLocation { parents: 1, interior: X1(Parachain(_)) })
	}
}


/// Estimates outgoing fees on target chain 
pub struct OutgoingFee<Registry :RemoteAssetRegistryInspect> {
	_marker: PhantomData<Registry>,
}


impl<Registry :RemoteAssetRegistryInspect< AssetId = CurrencyId, AssetNativeLocation = XcmAssetLocation, Balance = Balance>> OutgoingFee<Registry> {
	pub fn outgoing_fee(location: &MultiLocation) -> Option<Balance> {
		match (location.parents, location.first_interior()) {
			(1, None) => Some(400_000_000_000),
			(1, Some(Parachain(id)))  =>  {
				let location = XcmAssetLocation::new(location.clone());
				Registry::min_xcm_fee(ParaId::from(*id), location).or(Some(u128::MAX))
			},
			_ => None,
		}
	}
}