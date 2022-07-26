use frame_support::weights::Weight;

pub trait WeightInfo {
	fn upload() -> Weight;
	fn instantiate() -> Weight;
	fn execute() -> Weight;
}

impl WeightInfo for () {
	fn upload() -> Weight {
		10_000
	}

	fn instantiate() -> Weight {
		10_000
	}

	fn execute() -> Weight {
		10_000
	}
}
