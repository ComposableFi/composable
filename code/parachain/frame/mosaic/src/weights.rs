#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn set_relayer() -> Weight;
	fn rotate_relayer() -> Weight;
	fn set_network() -> Weight;
	fn set_budget() -> Weight;
	fn transfer_to() -> Weight;
	fn accept_transfer() -> Weight;
	fn claim_stale_to() -> Weight;
	fn timelocked_mint() -> Weight;
	fn set_timelock_duration() -> Weight;
	fn rescind_timelocked_mint() -> Weight;
	fn claim_to() -> Weight;
  fn update_asset_mapping() -> Weight;
  fn add_remote_amm_id() -> Weight;
  fn remove_remote_amm_id() -> Weight;
}

// For backwards compatibility and tests
impl WeightInfo for () {
  fn set_relayer() -> Weight {
    Weight::from_ref_time(10_000)
  }

  fn rotate_relayer() -> Weight {
    Weight::from_ref_time(10_000)
  }

  fn set_network() -> Weight {
    Weight::from_ref_time(10_000)
  }

  fn set_budget() -> Weight {
    Weight::from_ref_time(10_000)
  }

  fn transfer_to() -> Weight {
    Weight::from_ref_time(10_000)
  }

  fn accept_transfer() -> Weight {
    Weight::from_ref_time(10_000)
  }

  fn claim_stale_to() -> Weight {
    Weight::from_ref_time(10_000)
  }

  fn timelocked_mint() -> Weight {
    Weight::from_ref_time(10_000)
  }

  fn set_timelock_duration() -> Weight {
    Weight::from_ref_time(10_000)
  }

  fn rescind_timelocked_mint() -> Weight {
    Weight::from_ref_time(10_000)
  }

  fn claim_to() -> Weight {
    Weight::from_ref_time(10_000)
  }

  fn update_asset_mapping() -> Weight {
    Weight::from_ref_time(10_000)
  }

  fn add_remote_amm_id() -> Weight {
    Weight::from_ref_time(10_000)
  }
  fn remove_remote_amm_id() -> Weight {
    Weight::from_ref_time(10_000)
  }
}

