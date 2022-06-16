use frame_support::weights::Weight;
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn add_margin() -> Weight;
	fn withdraw_collateral() -> Weight;
	fn create_market() -> Weight;
	fn open_position() -> Weight;
	fn close_position() -> Weight;
	fn update_funding() -> Weight;
	fn liquidate() -> Weight;
}

/// Weights for `pallet_clearing_house` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn add_margin() -> Weight {
		1_000_u32.into()
	}

	fn withdraw_collateral() -> Weight {
		1_000_u32.into()
	}

	fn create_market() -> Weight {
		1_000_u32.into()
	}

	fn open_position() -> Weight {
		1_000_u32.into()
	}

	fn close_position() -> Weight {
		1_000_u32.into()
	}

	fn update_funding() -> Weight {
		1_000_u32.into()
	}

	fn liquidate() -> Weight {
		1_000_u32.into()
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn add_margin() -> Weight {
		1_000_u32.into()
	}

	fn withdraw_collateral() -> Weight {
		1_000_u32.into()
	}

	fn create_market() -> Weight {
		1_000_u32.into()
	}

	fn open_position() -> Weight {
		1_000_u32.into()
	}

	fn close_position() -> Weight {
		1_000_u32.into()
	}

	fn update_funding() -> Weight {
		1_000_u32.into()
	}

	fn liquidate() -> Weight {
		1_000_u32.into()
	}
}
