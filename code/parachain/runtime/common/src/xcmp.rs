//! proposed shared XCM setup parameters and impl
use crate::{fees::NativeBalance, prelude::*, AccountId, Balance};
use frame_support::{
	dispatch::Weight,
	log, match_types, parameter_types,
	traits::{tokens::BalanceConversion, Contains, Get},
	weights::{WeightToFee, WeightToFeePolynomial},
};
use num_traits::{One, Zero};
use orml_traits::location::{AbsoluteReserveProvider, Reserve};
use polkadot_primitives::v2::Id;
use primitives::currency::{CurrencyId, WellKnownCurrency};
use sp_runtime::traits::Convert;
use sp_std::marker::PhantomData;
use xcm::latest::{MultiAsset, MultiLocation};
use xcm_builder::*;
use xcm_executor::{
	traits::{FilterAssetLocation, WeightTrader},
	*,
};
pub const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;

match_types! {
	pub type ParentOrSiblings: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: Here } |
		MultiLocation { parents: 1, interior: X1(_) }
	};
}

parameter_types! {
	pub const BaseXcmWeight: Weight = Weight::from_ref_time(100_000_000);
	pub const XcmMaxAssetsForTransfer: usize = 2;
	pub RelayNativeLocation: MultiLocation = MultiLocation::parent();
	pub RelayOrigin: cumulus_pallet_xcm::Origin = cumulus_pallet_xcm::Origin::Relay;
	pub const UnitWeightCost: Weight = Weight::from_ref_time(200_000_000);
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
		origin == &topology::this::LOCAL || origin == &Self::self_parent()
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
		PriceConverter: BalanceConversion<NativeBalance, CurrencyId, Balance>,
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
			PriceConverter::to_asset_balance(fee, asset_id).map_err(|_| XcmError::TooExpensive)?;
		let price = price.max(Balance::one());
		log::trace!(target : "xcmp::weight_to_asset", "amount of priceable token to pay fee {:?}", price );
		Ok((fee, price))
	}
}
impl<
		AssetConverter: Convert<MultiLocation, Option<CurrencyId>>,
		PriceConverter: BalanceConversion<NativeBalance, CurrencyId, Balance>,
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
		let xcmp_asset_id = *payment
			.fungible
			.iter()
			.next()
			.map_or(Err(XcmError::TooExpensive), |v| Ok(v.0))?;

		if let AssetId::Concrete(ref multi_location) = xcmp_asset_id {
			if let Some(asset_id) = AssetConverter::convert(*multi_location) {
				let (fee, price) = Self::weight_to_asset(weight, asset_id)?;
				let required = MultiAsset { id: xcmp_asset_id, fun: Fungibility::Fungible(price) };
				log::trace!(target : "xcmp::buy_weight", "required priceable token {:?}; provided payment:{:?} ", required, payment );
				let unused = payment.checked_sub(required).map_err(|_| XcmError::TooExpensive)?;

				self.fee = self.fee.saturating_add(fee);
				self.price = self.price.saturating_add(price);
				self.asset_location = Some(*multi_location);
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
				return Some((*asset_location, price).into())
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
		X1(AccountId32 { network: None, id: account.into() }).into()
	}
}

pub struct CurrencyIdConvert<AssetRegistry, WellKnownCurrency, ThisParaId>(
	PhantomData<(AssetRegistry, WellKnownCurrency, ThisParaId)>,
);

impl<
		AssetRegistry: Convert<CurrencyId, Option<MultiLocation>>,
		WellKnown: WellKnownCurrency,
		ThisParaId: Get<Id>,
	> sp_runtime::traits::Convert<CurrencyId, Option<MultiLocation>>
	for CurrencyIdConvert<AssetRegistry, WellKnown, ThisParaId>
{
	fn convert(id: CurrencyId) -> Option<MultiLocation> {
		WellKnown::local_to_remote(id).or_else(|| AssetRegistry::convert(id))
	}
}

impl<
		AssetsRegistry: Convert<MultiLocation, Option<CurrencyId>>,
		WellKnown: WellKnownCurrency,
		ThisParaId: Get<Id>,
	> Convert<MultiLocation, Option<CurrencyId>>
	for CurrencyIdConvert<AssetsRegistry, WellKnown, ThisParaId>
{
	fn convert(location: MultiLocation) -> Option<CurrencyId> {
		log::trace!(target: "xcmp::convert", "converting {:?} on {:?}", &location, ThisParaId::get());
		match location {
			topology::relay::LOCATION => Some(WellKnown::RELAY_NATIVE),
			topology::this::LOCAL => Some(WellKnown::NATIVE),
			MultiLocation { parents, interior: X2(Parachain(id), GeneralIndex(index)) }
				if parents == 1 && Id::from(id) == ThisParaId::get() =>
				Some(CurrencyId(index)),
			MultiLocation { parents: 0, interior: X1(GeneralIndex(index)) } =>
				Some(CurrencyId(index)),
			_ =>
				if let Some(currency_id) = WellKnown::remote_to_local(location) {
					Some(currency_id)
				} else {
					log::trace!(target: "xcmp", "using assets registry for {:?}", location);
					let result = AssetsRegistry::convert(location).map(Into::into);
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

impl<
		T: Convert<MultiLocation, Option<CurrencyId>>,
		WellKnown: WellKnownCurrency,
		ThisParaId: Get<Id>,
	> Convert<MultiAsset, Option<CurrencyId>> for CurrencyIdConvert<T, WellKnown, ThisParaId>
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
	fn contains(asset: &MultiAsset, origin: &MultiLocation) -> bool {
		AbsoluteReserveProvider::reserve(asset) == Some(MultiLocation::parent()) &&
			matches!(origin, MultiLocation { parents: 1, interior: X1(Parachain(_)) })
	}
}

parameter_types! {
	pub const ThisLocal: MultiLocation = primitives::topology::this::LOCAL;
}
