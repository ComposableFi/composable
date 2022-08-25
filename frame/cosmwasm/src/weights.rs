use frame_support::weights::Weight;

pub trait WeightInfo {
	fn upload(code_len: usize) -> Weight;
	fn instantiate(nb_of_assets: usize) -> Weight;
	fn execute(nb_of_assets: usize) -> Weight;
}

impl WeightInfo for () {
	fn upload(_code_len: usize) -> Weight {
		10_000
	}

	fn instantiate(_nb_of_assets: usize) -> Weight {
		10_000
	}

	fn execute(_nb_of_assets: usize) -> Weight {
		10_000
	}
}
