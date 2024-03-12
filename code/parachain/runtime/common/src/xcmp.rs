//! proposed shared XCM setup parameters and impl
use crate::{fees::NativeBalance, prelude::*, AccountId, Balance};
use frame_support::{
	dispatch::Weight,
	log, match_types, parameter_types,
	traits::{tokens::ConversionToAssetBalance, Contains, Get, PalletInfoAccess},
	weights::{WeightToFee, WeightToFeePolynomial},
};
use num_traits::{One, Zero};
use orml_traits::location::{AbsoluteReserveProvider, Reserve};
use polkadot_primitives::v4::Id;
use primitives::currency::{CurrencyId, WellKnownCurrency};
use sp_runtime::traits::Convert;
use sp_std::marker::PhantomData;
use xcm::latest::{MultiAsset, MultiLocation};
use xcm_builder::*;
use xcm_executor::{traits::WeightTrader, *};
pub const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;

match_types! {
	pub type ParentOrSiblings: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: Here } |
		MultiLocation { parents: 1, interior: X1(_) }
	};
}

parameter_types! {
	pub const BaseXcmWeight: Weight = Weight::from_parts(100_000_000, 0);
	pub const XcmMaxAssetsForTransfer: usize = 2;
	pub RelayNativeLocation: MultiLocation = MultiLocation::parent();
	pub RelayOrigin: cumulus_pallet_xcm::Origin = cumulus_pallet_xcm::Origin::Relay;
	pub const UnitWeightCost: Weight = Weight::from_parts(200_000_000, 0);
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
		PriceConverter: ConversionToAssetBalance<NativeBalance, CurrencyId, Balance>,
		Treasury: TakeRevenue,
		WeightToFeeConverter: WeightToFeePolynomial<Balance = Balance> + WeightToFee<Balance = Balance>,
	> TransactionFeePoolTrader<AssetConverter, PriceConverter, Treasury, WeightToFeeConverter>
{
	pub fn weight_to_asset(
		weight: xcm::latest::Weight,
		asset_id: CurrencyId,
	) -> Result<(Balance, Balance), XcmError> {
		let fee = WeightToFeeConverter::weight_to_fee(&Weight::from_parts(
			weight.ref_time(),
			weight.proof_size(),
		));
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
		PriceConverter: ConversionToAssetBalance<NativeBalance, CurrencyId, Balance>,
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

	fn buy_weight(
		&mut self,
		weight: xcm::latest::Weight,
		payment: Assets,
	) -> Result<Assets, XcmError> {
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

	fn refund_weight(&mut self, weight: xcm::latest::Weight) -> Option<MultiAsset> {
		if let Some(ref asset_location) = self.asset_location {
			let fee = WeightToFeeConverter::weight_to_fee(&Weight::from_parts(
				weight.ref_time(),
				weight.proof_size(),
			));
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

pub struct CurrencyIdConvert<ForeignAssetsToXcm, WellKnownCurrency, AssetsRegistry, ThisParaId>(
	PhantomData<(ForeignAssetsToXcm, WellKnownCurrency, AssetsRegistry, ThisParaId)>,
);

impl<
		ForeignAssetsToXcm: Convert<CurrencyId, Option<MultiLocation>>,
		WellKnown: WellKnownCurrency,
		AssetsRegistry: PalletInfoAccess,
		ThisParaId: Get<Id>,
	> sp_runtime::traits::Convert<CurrencyId, Option<MultiLocation>>
	for CurrencyIdConvert<ForeignAssetsToXcm, WellKnown, AssetsRegistry, ThisParaId>
{
	fn convert(id: CurrencyId) -> Option<MultiLocation> {
		WellKnown::local_to_remote(id).or_else(|| ForeignAssetsToXcm::convert(id))
	}
}

impl<
		ForeignAssetsToXcm: Convert<MultiLocation, Option<CurrencyId>>,
		WellKnown: WellKnownCurrency,
		AssetsRegistry: PalletInfoAccess,
		ThisParaId: Get<Id>,
	> Convert<MultiLocation, Option<CurrencyId>>
	for CurrencyIdConvert<ForeignAssetsToXcm, WellKnown, AssetsRegistry, ThisParaId>
{
	fn convert(location: MultiLocation) -> Option<CurrencyId> {
		log::trace!(target: "xcmp::convert", "converting {:?} on {:?}", &location, ThisParaId::get());
		match location {
			topology::relay::LOCATION => Some(WellKnown::RELAY_NATIVE),
			topology::this::LOCAL => Some(WellKnown::NATIVE),
			MultiLocation {
				parents,
				interior: X3(Parachain(id), PalletInstance(pallet_index), GeneralIndex(index)),
			} if parents == 1 &&
				Id::from(id) == ThisParaId::get() &&
				pallet_index == AssetsRegistry::index() as u8 =>
				Some(CurrencyId(index)),
			MultiLocation {
				parents,
				interior: X2(PalletInstance(pallet_index), GeneralIndex(index)),
			} if parents == 0 && pallet_index == AssetsRegistry::index() as u8 => Some(CurrencyId(index)),
			_ =>
				if let Some(currency_id) = WellKnown::remote_to_local(location) {
					Some(currency_id)
				} else {
					log::trace!(target: "xcmp", "using assets registry for {:?}", location);
					let result = ForeignAssetsToXcm::convert(location).map(Into::into);
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
		ForeignAssetsToXcm: Convert<MultiLocation, Option<CurrencyId>>,
		WellKnown: WellKnownCurrency,
		AssetsRegistry: PalletInfoAccess,
		ThisParaId: Get<Id>,
	> Convert<MultiAsset, Option<CurrencyId>>
	for CurrencyIdConvert<ForeignAssetsToXcm, WellKnown, AssetsRegistry, ThisParaId>
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

#[allow(deprecated)]
impl xcm_executor::traits::FilterAssetLocation for RelayReserveFromParachain {
	fn contains(asset: &MultiAsset, origin: &xcm::latest::MultiLocation) -> bool {
		AbsoluteReserveProvider::reserve(asset) == Some(xcm::latest::MultiLocation::parent()) &&
			matches!(
				origin,
				xcm::latest::MultiLocation { parents: 1, interior: X1(Parachain(_)) }
			)
	}
}

parameter_types! {
	pub const ThisLocal: MultiLocation = primitives::topology::this::LOCAL;
}

// TODO: Remove after upgrading to `polkadot-v1.2.0` and replace types from xcm-builder.

use codec::{Compact, Encode};
use sp_io::hashing::blake2_256;
use xcm_executor::traits::Convert as XcmConvert;

/// Means of converting a location into a stable and unique descriptive identifier.
pub trait DescribeLocation {
	/// Create a description of the given `location` if possible. No two locations should have the
	/// same descriptor.
	fn describe_location(location: &MultiLocation) -> Option<Vec<u8>>;
}

#[impl_trait_for_tuples::impl_for_tuples(30)]
impl DescribeLocation for Tuple {
	fn describe_location(l: &MultiLocation) -> Option<Vec<u8>> {
		for_tuples!( #(
			match Tuple::describe_location(l) {
				Some(result) => return Some(result),
				None => {},
			}
		)* );
		None
	}
}

pub struct DescribeTerminus;
impl DescribeLocation for DescribeTerminus {
	fn describe_location(l: &MultiLocation) -> Option<Vec<u8>> {
		match (l.parents, &l.interior) {
			(0, Here) => Some(Vec::new()),
			_ => None,
		}
	}
}

pub struct DescribePalletTerminal;
impl DescribeLocation for DescribePalletTerminal {
	fn describe_location(l: &MultiLocation) -> Option<Vec<u8>> {
		match (l.parents, &l.interior) {
			(0, X1(PalletInstance(i))) =>
				Some((b"Pallet", Compact::<u32>::from(*i as u32)).encode()),
			_ => None,
		}
	}
}

pub struct DescribeAccountId32Terminal;
impl DescribeLocation for DescribeAccountId32Terminal {
	fn describe_location(l: &MultiLocation) -> Option<Vec<u8>> {
		match (l.parents, &l.interior) {
			(0, X1(AccountId32 { id, .. })) => Some((b"AccountId32", id).encode()),
			_ => None,
		}
	}
}

pub struct DescribeAccountKey20Terminal;
impl DescribeLocation for DescribeAccountKey20Terminal {
	fn describe_location(l: &MultiLocation) -> Option<Vec<u8>> {
		match (l.parents, &l.interior) {
			(0, X1(AccountKey20 { key, .. })) => Some((b"AccountKey20", key).encode()),
			_ => None,
		}
	}
}

pub type DescribeAccountIdTerminal = (DescribeAccountId32Terminal, DescribeAccountKey20Terminal);

pub struct DescribeBodyTerminal;
impl DescribeLocation for DescribeBodyTerminal {
	fn describe_location(l: &MultiLocation) -> Option<Vec<u8>> {
		match (l.parents, &l.interior) {
			(0, X1(Plurality { id, part })) => Some((b"Body", id, part).encode()),
			_ => None,
		}
	}
}

pub type DescribeAllTerminal = (
	DescribeTerminus,
	DescribePalletTerminal,
	DescribeAccountId32Terminal,
	DescribeAccountKey20Terminal,
	DescribeBodyTerminal,
);

pub struct DescribeFamily<DescribeInterior>(PhantomData<DescribeInterior>);
impl<Suffix: DescribeLocation> DescribeLocation for DescribeFamily<Suffix> {
	fn describe_location(l: &MultiLocation) -> Option<Vec<u8>> {
		match (l.parents, l.interior.first()) {
			(0, Some(Parachain(index))) => {
				let tail = l.interior.split_first().0;
				let interior = Suffix::describe_location(&tail.into())?;
				Some((b"ChildChain", Compact::<u32>::from(*index), interior).encode())
			},
			(1, Some(Parachain(index))) => {
				let tail = l.interior.split_first().0;
				let interior = Suffix::describe_location(&tail.into())?;
				Some((b"SiblingChain", Compact::<u32>::from(*index), interior).encode())
			},
			(1, _) => {
				let tail = l.interior.into();
				let interior = Suffix::describe_location(&tail)?;
				Some((b"ParentChain", interior).encode())
			},
			_ => None,
		}
	}
}

pub struct HashedDescription<AccountId, Describe>(PhantomData<(AccountId, Describe)>);
impl<AccountId: From<[u8; 32]> + Clone, Describe: DescribeLocation>
	XcmConvert<MultiLocation, AccountId> for HashedDescription<AccountId, Describe>
{
	fn convert(value: MultiLocation) -> Result<AccountId, MultiLocation> {
		if let Some(description) = Describe::describe_location(&value) {
			Ok(blake2_256(&description).into())
		} else {
			Err(value)
		}
	}
}
