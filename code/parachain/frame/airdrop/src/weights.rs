use frame_support::weights::Weight;

pub trait WeightInfo {
	fn create_airdrop() -> Weight;
	fn add_recipient(x: u32) -> Weight;
	fn remove_recipient() -> Weight;
	fn enable_airdrop() -> Weight;
	fn disable_airdrop() -> Weight;
	fn claim(x: u32) -> Weight;
}

impl WeightInfo for () {
	fn create_airdrop() -> Weight {
		Weight::from_ref_time(10_000)
	}

	fn add_recipient(_x: u32) -> Weight {
		Weight::from_ref_time(10_000)
	}

	fn remove_recipient() -> Weight {
		Weight::from_ref_time(10_000)
	}

	fn enable_airdrop() -> Weight {
		Weight::from_ref_time(10_000)
	}

	fn disable_airdrop() -> Weight {
		Weight::from_ref_time(10_000)
	}

	fn claim(_x: u32) -> Weight {
		Weight::from_ref_time(10_000)
	}
}
