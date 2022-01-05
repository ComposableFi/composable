use frame_support::dispatch::Weight;

pub trait WeightInfo {
	fn ask() -> Weight;
	fn take() -> Weight;
	fn liquidate() -> Weight;
	fn known_overhead_for_on_finalize() -> Weight;
}

impl WeightInfo for () {
	fn ask() -> Weight {
		0
	}

	fn take() -> Weight {
		0
	}

	fn liquidate() -> Weight {
		0
	}

	fn known_overhead_for_on_finalize() -> Weight {
		0
	}
}
