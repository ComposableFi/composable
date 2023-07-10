//! Benchmarks and  sanity tests for lending. Only test that action do not error, not that produce
//! positive side effects

use super::*;
use crate::{self as pallet_assets_interface, runtime::*};

#[allow(unused_imports)]
use crate::Pallet as AssetsInterface;

use composable_traits::{
	assets::{BiBoundedAssetName, BiBoundedAssetSymbol, GenerateAssetId},
	storage::UpdateValue,
};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use primitives::currency::{ForeignAssetId, VersionedMultiLocation};
use runtime::AssetsRegistry;
use sp_std::prelude::*;
use xcm::{
	latest::MultiLocation,
	v3::{Junction::Parachain, Junctions::X1},
};

use frame_support::traits::{fungible::Mutate, PalletInfoAccess};

benchmarks! {
	where_clause {
		where
			T: pallet_assets_interface::Config + frame_system::Config,
			<T as pallet_assets_interface::Config>::Balance : From<u64>,
			<T as pallet_assets_interface::Config>::AssetId : From<u128>,
			<T as frame_system::Config>::AccountId : Into<u128>,
	}

	register_asset {
		let owner: T::AccountId = whitelisted_caller();
		let location = Some(T::Location::decode(&mut ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::here())).encode().as_ref()).unwrap());
		let name = Some(BiBoundedAssetName::from_vec(b"Kusama".to_vec()).expect("String is within bounds"));
		let symbol= Some(BiBoundedAssetSymbol::from_vec(b"KSM".to_vec()).expect("Stringis within bounds"));
		let decimals= 3;
		Balances::mint_into(&T::AccountId::into(owner.clone()), 200)?;
		AssetsInterface::<T>::set_creation_fee(
			RawOrigin::Root.into(),
			T::Balance::from(100)
		)?;
	}: _(RawOrigin::Signed(owner), location, name, symbol, decimals)

	register_cosmwasm_asset {
		let owner: T::AccountId = whitelisted_caller();
		let name = Some(BiBoundedAssetName::from_vec(b"Kusama".to_vec()).expect("String is within bounds"));
		let symbol= Some(BiBoundedAssetSymbol::from_vec(b"KSM".to_vec()).expect("Stringis within bounds"));
		let decimals= 3;
		Balances::mint_into(&T::AccountId::into(owner.clone()), 200)?;
		AssetsInterface::<T>::set_creation_fee(
			RawOrigin::Root.into(),
			T::Balance::from(100)
		)?;
	}: _(RawOrigin::Signed(owner), name, symbol, decimals)

	set_creation_fee {
		let owner = ALICE;
		let fee_amount: T::Balance = T::Balance::from(100_000);
	}: _(RawOrigin::Root, fee_amount)

	update_asset {
		let owner: T::AccountId = whitelisted_caller();
		let location = T::Location::decode(&mut ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::here())).encode().as_ref()).unwrap();
		let name = Some(BiBoundedAssetName::from_vec(b"Kusama".to_vec()).expect("String is within bounds"));
		let symbol= Some(BiBoundedAssetSymbol::from_vec(b"KSM".to_vec()).expect("Stringis within bounds"));
		let decimals= 3;
		Balances::mint_into(&T::AccountId::into(owner.clone()), 200)?;
		AssetsInterface::<T>::set_creation_fee(
			RawOrigin::Root.into(),
			T::Balance::from(100)
		)?;

		AssetsInterface::<T>::register_asset(
			RawOrigin::Signed(owner.clone()).into(),
			Some(location.clone()),
			name,
			symbol,
			decimals,
		)
		.expect("Asset details are non-duplicate and valid");

		let location = <Test as pallet_assets_registry::Config>::ForeignAssetId::decode(&mut ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::here())).encode().as_ref()).unwrap();
		let name = UpdateValue::Set(Some(BiBoundedAssetName::from_vec(b"Cooler Kusama".to_vec()).expect("String is within bounds")));
		let symbol= UpdateValue::Set(Some(BiBoundedAssetSymbol::from_vec(b"CKSM".to_vec()).expect("Stringis within bounds")));
		let decimals= UpdateValue::Set(4);
		let asset_id: T::AssetId = <AssetsRegistry as GenerateAssetId>::generate_asset_id((AssetsInterface::<T>::index() as u32).to_be_bytes(), AssetNonce::<T>::get()).into();
	}: _(RawOrigin::Signed(owner), asset_id, name, symbol, decimals)

	update_asset_location {
		let owner: T::AccountId = whitelisted_caller();
		let location = T::Location::decode(&mut ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::here())).encode().as_ref()).unwrap();
		let name = Some(BiBoundedAssetName::from_vec(b"Kusama".to_vec()).expect("String is within bounds"));
		let symbol= Some(BiBoundedAssetSymbol::from_vec(b"KSM".to_vec()).expect("Stringis within bounds"));
		let decimals= 3;
		Balances::mint_into(&T::AccountId::into(owner.clone()), 200)?;
		AssetsInterface::<T>::set_creation_fee(
			RawOrigin::Root.into(),
			T::Balance::from(100)
		)?;

		AssetsInterface::<T>::register_asset(
			RawOrigin::Signed(owner.clone()).into(),
			Some(location.clone()),
			name,
			symbol,
			decimals,
		)
		.expect("Asset details are non-duplicate and valid");
		let location = <Test as pallet_assets_registry::Config>::ForeignAssetId::decode(&mut ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::here())).encode().as_ref()).unwrap();


		let asset_id: T::AssetId = <AssetsRegistry as RemoteAssetRegistryInspect>::location_to_asset(location)
			.expect("Asset exists").into();

		let location_new = T::Location::decode(&mut ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::new(1, X1(Parachain(4321))))).encode().as_ref()).unwrap();
	}: _(RawOrigin::Signed(owner), asset_id, Some(location_new))

	mint_cosmwasm {
		let owner: T::AccountId = whitelisted_caller();
		let location = T::Location::decode(&mut ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::here())).encode().as_ref()).unwrap();
		let name = Some(BiBoundedAssetName::from_vec(b"Kusama".to_vec()).expect("String is within bounds"));
		let symbol= Some(BiBoundedAssetSymbol::from_vec(b"KSM".to_vec()).expect("Stringis within bounds"));
		let decimals= 3;
		Balances::mint_into(&T::AccountId::into(owner.clone()), 200)?;
		AssetsInterface::<T>::set_creation_fee(
			RawOrigin::Root.into(),
			T::Balance::from(100)
		)?;

		AssetsInterface::<T>::register_cosmwasm_asset(
			RawOrigin::Signed(owner.clone()).into(),
			name,
			symbol,
			decimals,
		)
		.expect("Asset details are non-duplicate and valid");


		let asset_id: T::AssetId = <AssetsRegistry as GenerateAssetId>::generate_asset_id((AssetsInterface::<T>::index() as u32).to_be_bytes(), AssetNonce::<T>::get()).into();
	}: _(RawOrigin::Signed(owner.clone()), asset_id, owner.clone(), T::Balance::from(100_000))

	burn_cosmwasm {
		let owner: T::AccountId = whitelisted_caller();
		let location = T::Location::decode(&mut ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::here())).encode().as_ref()).unwrap();
		let name = Some(BiBoundedAssetName::from_vec(b"Kusama".to_vec()).expect("String is within bounds"));
		let symbol= Some(BiBoundedAssetSymbol::from_vec(b"KSM".to_vec()).expect("Stringis within bounds"));
		let decimals= 3;
		AssetsInterface::<T>::set_creation_fee(
			RawOrigin::Root.into(),
			T::Balance::from(100)
		)?;
		Balances::mint_into(&T::AccountId::into(owner.clone()), 200)?;
		AssetsInterface::<T>::register_cosmwasm_asset(
			RawOrigin::Signed(owner.clone()).into(),
			name,
			symbol,
			decimals,
		)
		.expect("Asset details are non-duplicate and valid");
		let asset_id: T::AssetId = <AssetsRegistry as GenerateAssetId>::generate_asset_id((AssetsInterface::<T>::index() as u32).to_be_bytes(), AssetNonce::<T>::get()).into();
		AssetsInterface::<T>::mint_cosmwasm(
			RawOrigin::Signed(owner.clone()).into(),
			asset_id,
			owner.clone(),
			T::Balance::from(200_000),
		)
		.expect("Asset details are non-duplicate and valid");

	}: _(RawOrigin::Signed(owner.clone()), asset_id, owner.clone(), T::Balance::from(100_000))
}

impl_benchmark_test_suite!(AssetsInterface, crate::runtime::new_test_ext(), crate::runtime::Test);
