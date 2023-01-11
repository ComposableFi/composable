#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{RefTimeWeight, Weight, constants::RocksDbWeight}};
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
    10_000 as Weight
  }

  fn rotate_relayer() -> Weight {
    10_000 as Weight
  }

  fn set_network() -> Weight {
    10_000 as Weight
  }

  fn set_budget() -> Weight {
    10_000 as Weight
  }

  fn transfer_to() -> Weight {
    10_000 as Weight
  }

  fn accept_transfer() -> Weight {
    10_000 as Weight
  }

  fn claim_stale_to() -> Weight {
    10_000 as Weight
  }

  fn timelocked_mint() -> Weight {
    10_000 as Weight
  }

  fn set_timelock_duration() -> Weight {
    10_000 as Weight
  }

  fn rescind_timelocked_mint() -> Weight {
    10_000 as Weight
  }

  fn claim_to() -> Weight {
    10_000 as Weight
  }

  fn update_asset_mapping() -> Weight {
    10_000 as Weight
  }

  fn add_remote_amm_id() -> Weight {
    10_000 as Weight
  }
  fn remove_remote_amm_id() -> Weight {
    10_000 as Weight
  }
}

