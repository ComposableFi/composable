//! Benchmarks and  sanity tests for lending. Only test that action do not error, not that produce
//! positive side effects

use super::*;
use crate::{self as pallet_assets_registry, Pallet as AssetsRegistry};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_system::{EventRecord, RawOrigin};
use sp_std::prelude::*;

const SEED: u32 = 0;

#[allow(dead_code)]
pub fn assert_last_event<T: pallet_assets_registry::Config>(generic_event: <T as Config>::Event) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::Event = generic_event.into();
	// compare to the last event record
	let EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

benchmarks! {
	where_clause {
		where
			T: pallet_assets_registry::Config + frame_system::Config
	}

	set_local_admin {
		let local_admin: T::AccountId = account("local_admin", 0, SEED);
	} : _(RawOrigin::Root, local_admin.clone())
	verify {
		assert_last_event::<T>(Event::LocalAdminUpdated(local_admin).into())
	}

	set_foreign_admin {
		let foreign_admin: T::AccountId = account("foreign_admin", 0, SEED);
	} : _(RawOrigin::Root, foreign_admin.clone())
	verify {
		assert_last_event::<T>(Event::ForeignAdminUpdated(foreign_admin).into())
	}

	approve_assets_mapping_candidate {
		let root = RawOrigin::Root.into();
		let local_admin: T::AccountId = account("local_admin", 0, SEED);
		let (local_asset_id, foreign_asset_id, location, decimals): (T::LocalAssetId, T::ForeignAssetId, T::Location, u8) =
			(1.into(), 100.into(), Default::default(), 12);
		AssetsRegistry::<T>::set_local_admin(root, local_admin.clone()).expect("Could not set local admin");
	}: _(RawOrigin::Signed(local_admin), local_asset_id, foreign_asset_id, location, decimals)
	verify {
		assert_last_event::<T>(Event::AssetsMappingCandidateUpdated { local_asset_id, foreign_asset_id }.into())
	}

	set_metadata {
		let root: <T as frame_system::Config>::Origin = RawOrigin::Root.into();
		let local_admin: T::AccountId = account("local_admin", 0, SEED);
		let foreign_admin: T::AccountId = account("foreign_admin", 0, SEED);
		let (local_asset_id, foreign_asset_id, location, decimals): (T::LocalAssetId, T::ForeignAssetId, T::Location, u8) =
			(1.into(), 100.into(), Default::default(), 12);
		let foreign_metadata = ForeignMetadata { decimals };
		AssetsRegistry::<T>::set_local_admin(root.clone(), local_admin.clone()).expect("Could not set local admin");
		AssetsRegistry::<T>::set_foreign_admin(root, foreign_admin.clone()).expect("Could not set foreign admin");
		AssetsRegistry::<T>::approve_assets_mapping_candidate(
			RawOrigin::Signed(local_admin.clone()).into(),
			local_asset_id,
			foreign_asset_id,
			location.clone(),
			decimals
		).expect("Could not approve assets mapping candidate");
		AssetsRegistry::<T>::approve_assets_mapping_candidate(
			RawOrigin::Signed(foreign_admin).into(),
			local_asset_id,
			foreign_asset_id,
			location,
			decimals
		).expect("Could not approve assets mapping candidate");
	}: _(RawOrigin::Signed(local_admin), local_asset_id, foreign_metadata)
	verify {
		assert_last_event::<T>(Event::AssetMetadataUpdated(local_asset_id).into())
	}
}

impl_benchmark_test_suite!(AssetsRegistry, crate::mock::new_test_ext(), crate::mock::Test);
