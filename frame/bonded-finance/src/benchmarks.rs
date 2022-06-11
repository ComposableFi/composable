#![cfg(feature = "runtime-benchmarks")]

#[cfg(test)]
use crate::Pallet as BondedFinance;
use crate::{AssetIdOf, BalanceOf, BlockNumberOf, BondOfferOf, Call, Config, Pallet};
use codec::Decode;
use composable_support::validation::Validated;
use composable_traits::bonded_finance::{BondDuration, BondOffer, BondOfferReward};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{
	dispatch::UnfilteredDispatchable,
	traits::{fungible::Mutate as _, fungibles::Mutate as _},
};
use frame_system::RawOrigin;
use sp_runtime::traits::One;

const MIN_VESTED_TRANSFER: u128 = 1000 * 1_000_000_000_000;
const BALANCE: u128 = 1_000_000 * 1_000_000_000_000;

fn assets<T>() -> [AssetIdOf<T>; 2]
where
	T: Config,
{
	let a = 1u128.to_be_bytes();
	let b = 2u128.to_be_bytes();
	[AssetIdOf::<T>::decode(&mut &a[..]).unwrap(), AssetIdOf::<T>::decode(&mut &b[..]).unwrap()]
}

fn bond_offer<T>(bond_asset: AssetIdOf<T>, reward_asset: AssetIdOf<T>) -> BondOfferOf<T>
where
	T: Config,
	BalanceOf<T>: From<u128>,
{
	BondOffer {
		beneficiary: whitelisted_caller(),
		asset: bond_asset,
		bond_price: BalanceOf::<T>::from(MIN_VESTED_TRANSFER),
		maturity: BondDuration::Finite { return_in: BlockNumberOf::<T>::from(1u32) },
		nb_of_bonds: BalanceOf::<T>::from(1u128),
		reward: BondOfferReward {
			amount: BalanceOf::<T>::from(MIN_VESTED_TRANSFER),
			asset: reward_asset,
			maturity: BlockNumberOf::<T>::from(96u32),
		},
	}
}

fn call_bond<T>(caller: &T::AccountId, nb_of_bonds: BalanceOf<T>, offer_id: T::BondOfferId)
where
	T: Config,
{
	let offer_account_id = Pallet::<T>::account_id(offer_id);
	let keep_alive = false;
	T::NativeCurrency::mint_into(&offer_account_id, <_>::try_from(BALANCE).unwrap_or_default())
		.unwrap();
	Call::<T>::bond { nb_of_bonds, offer_id, keep_alive }
		.dispatch_bypass_filter(RawOrigin::Signed(caller.clone()).into())
		.unwrap();
}

fn call_offer<T>(bond_offer: BondOfferOf<T>, caller: &T::AccountId)
where
	T: Config,
{
	let keep_alive = false;
	let validated_bond_offer = Validated::new(bond_offer).unwrap();
	Call::<T>::offer { offer: validated_bond_offer, keep_alive }
		.dispatch_bypass_filter(RawOrigin::Signed(caller.clone()).into())
		.unwrap();
}

fn initial_mint<T>(bond_asset: AssetIdOf<T>, caller: &T::AccountId, reward_assert: AssetIdOf<T>)
where
	T: Config,
{
	T::Currency::mint_into(bond_asset, caller, <_>::try_from(BALANCE).unwrap_or_default()).unwrap();
	T::Currency::mint_into(reward_assert, caller, <_>::try_from(BALANCE).unwrap_or_default())
		.unwrap();
	T::NativeCurrency::mint_into(caller, <_>::try_from(BALANCE).unwrap_or_default()).unwrap();
}

benchmarks! {
  where_clause {
	  where BalanceOf<T>: From<u128>
  }

	offer {
		let [bond_asset, reward_asset] = assets::<T>();
		let caller: T::AccountId = account("caller", 0, 0xCAFEBABE);
		initial_mint::<T>(bond_asset, &caller, reward_asset);
		let bond_offer = bond_offer::<T>(bond_asset, reward_asset);
		let validated_bond_offer = Validated::new(bond_offer).unwrap();
	}: _(RawOrigin::Signed(caller), validated_bond_offer, false)

	bond {
		let [bond_asset, reward_asset] = assets::<T>();
		let caller: T::AccountId = account("caller", 0, 0xCAFEBABE);
		initial_mint::<T>(bond_asset, &caller, reward_asset);
		let bond_offer = bond_offer::<T>(bond_asset, reward_asset);
		let nb_of_bonds = bond_offer.nb_of_bonds;
		call_offer::<T>(bond_offer, &caller);
		let offer_id = T::BondOfferId::one();
	}: _(RawOrigin::Signed(caller), offer_id, nb_of_bonds, false)

	cancel {
		let [bond_asset, reward_asset] = assets::<T>();
		let caller: T::AccountId = account("caller", 0, 0xCAFEBABE);
		initial_mint::<T>(bond_asset, &caller, reward_asset);
		let bond_offer = bond_offer::<T>(bond_asset, reward_asset);
		let nb_of_bonds = bond_offer.nb_of_bonds;
		call_offer::<T>(bond_offer, &caller);
		let offer_id = T::BondOfferId::one();
		call_bond::<T>(&caller, nb_of_bonds, offer_id);
	}: _(RawOrigin::Signed(caller), offer_id)
}

impl_benchmark_test_suite!(BondedFinance, crate::mock::ExtBuilder::build(), crate::mock::Runtime);
